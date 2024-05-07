use std::{
    collections::{HashMap, HashSet},
    os::unix::process::parent_id,
    str::FromStr,
};

use async_trait::async_trait;
use bytes::BytesMut;
use chrono::{DateTime, Utc};
use deadpool_postgres::{Client, GenericClient, Pool};
use postgres_types::{to_sql_checked, ToSql};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use tracing::info;

use crate::{
    constants::{self, MAGIC_RECYCLE_BIN},
    dbbackup::v1::{rowmapper::row_to_table, table::Table, BackupContent, BackupHandlerV1, RowSum},
    mapper::node::NodeInsertResult,
    model::{
        asset::Asset,
        node::{ContentParsedInfo, Node, NodeId, NodeType},
    },
};

use super::{
    asset::AssetMapper,
    node::{NodeDeleteReq, NodeMapper, NodeMoveReq, NodeMoveRsp, NodeRelation, NodeRenameReq},
    nodefilter::NodeFetchReq,
    Mapper,
};

#[derive(Debug, Deserialize, Clone)]
pub struct PostgresConfig {
    user: String,
    pass: String,
    dbname: String,
    host: String,
    port: u16,
}

impl Into<deadpool_postgres::Config> for PostgresConfig {
    fn into(self) -> deadpool_postgres::Config {
        let mut cfg = deadpool_postgres::Config::new();
        cfg.user = Some(self.user);
        cfg.password = Some(self.pass);
        cfg.dbname = Some(self.dbname);
        cfg.host = Some(self.host);
        cfg.port = Some(self.port);
        cfg
    }
}

pub struct PostgresMapper {
    pub pool: Pool,
}

impl PostgresMapper {
    pub fn new(config: PostgresConfig) -> anyhow::Result<PostgresMapper> {
        let pool = Into::<deadpool_postgres::Config>::into(config)
            .create_pool(None, tokio_postgres::NoTls)?;

        Ok(PostgresMapper { pool })
    }

    async fn get_client(&self) -> anyhow::Result<Client> {
        self.pool.get().await.map_err(anyhow::Error::new)
    }

    async fn check_if_table_not_exists(&self, table_name: &str) -> anyhow::Result<bool> {
        let client = self.get_client().await?;
        client
            .query(
                "SELECT 1 FROM pg_tables WHERE  schemaname = 'public' AND tablename = $1",
                &[&table_name.to_string()],
            )
            .await
            .map(|e| e.is_empty())
            .map_err(anyhow::Error::new)
    }

    async fn create_table(&self, table_name: &str, create_sql: &str) -> anyhow::Result<()> {
        info!("begin to create table `{}'", table_name);
        if self.check_if_table_not_exists(table_name).await? {
            info!("table `{}' is not existed.", table_name);
            let client = self.get_client().await?;
            client
                .execute(create_sql, &[])
                .await
                .map(|_| ())
                .map_err(anyhow::Error::new)?;
        } else {
            info!("table {} is already created.", table_name);
        }
        Ok(())
    }

    fn map_row_node(row: &Row) -> Node {
        let id = row.get("id");
        Node {
            id,
            name: row.get("name"),
            content: row.try_get("content").map_or("".to_owned(), |e| e),
            user: row.get("username"),
            parsed_info: ContentParsedInfo::default(),
            parent_id: row.get("parent_id"),
            prev_sliding_id: row.get("prev_sliding_id"),
            delete_time: row.get("delete_time"),
            version_time: row.get("version_time"),
            initial_time: row.get("initial_time"),
            node_type: NodeType::from_str(row.get("node_type")).unwrap(),
        }
    }
}

#[async_trait]
impl AssetMapper for PostgresMapper {
    async fn insert_asset(
        &self,
        ori_file_name: &str,
        id: String,
        content_type: String,
        username: Option<String>,
    ) -> anyhow::Result<Asset> {
        let stmt = self.pool.get().await?;

        let create_time = Utc::now().to_owned();

        stmt.execute(
            "insert into assets(id, username, ori_file_name, content_type, create_time) values ($1,$2,$3,$4, $5)",
            &[&id, &username, &ori_file_name, &content_type, &create_time],
        )
        .await
        .map_err(|e| anyhow::Error::new(e))
        .map(|_| Asset {
            id,
            username,
            ori_file_name: ori_file_name.to_string(),
            content_type,
            create_time,
        })
    }

    async fn query_asset_by_id(&self, id: &str) -> anyhow::Result<Asset> {
        let stmt = self.pool.get().await?;
        let row = stmt
            .query_one("select * from assets where id = $1", &[&id])
            .await?;

        Ok(Asset {
            id: row.get("id"),
            username: row.get("username"),
            ori_file_name: row.get("ori_file_name"),
            content_type: row.get("content_type"),
            create_time: row.get("create_time"),
        })
    }
}

#[async_trait]
impl NodeMapper for PostgresMapper {
    async fn insert_node_only(&self, node: &Node) -> anyhow::Result<NodeInsertResult> {
        let stmt = self.pool.get().await?;

        let content: Result<String, tokio_postgres::Error> = stmt
            .query_one("select content from nodes where id = $1", &[&node.id])
            .await
            .map(|row| row.get("content"));

        let changed = match content {
            Ok(content) => distance::levenshtein(&content, &node.content) > 8,
            Err(_) => true,
        };

        if !changed {
            stmt.execute(
                "update nodes set content = $1, version_time = $2 where id = $3",
                &[&node.content, &node.version_time, &node.id],
            )
            .await?;
        } else {
            self._move_node_to_history(&node.id).await?;
            info!("begin to really insert");

            stmt
                .execute(
                    "insert into nodes(id, name, content, node_type, username, parent_id, prev_sliding_id, version_time, initial_time, delete_time) values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
                    &[
                        &node.id,
                        &node.name,
                        &node.content,
                        &node.node_type.as_ref(),
                        &node.user,
                        &node.parent_id,
                        &node.prev_sliding_id,
                        &node.version_time,
                        &node.initial_time,
                        &node.delete_time,
                    ]
                ).await
                .map_err(|e| { anyhow::Error::new(e) })
                .map(|_| ())?;
        }

        Ok(NodeInsertResult::ParsedInfo(ContentParsedInfo::default()))
    }

    async fn delete_node(&self, req: &NodeDeleteReq) -> anyhow::Result<()> {
        let stmt = self.pool.get().await?;

        let descendants = self.find_descendant_ids(&req.id).await?;

        let mut all_ids: HashSet<&NodeId> = descendants.keys().collect();
        descendants.iter().for_each(|(_, v)| {
            if let Some(key) = v.as_ref() {
                all_ids.insert(key);
            }
        });

        all_ids.insert(&req.id);

        let phs: String = all_ids
            .iter()
            .enumerate()
            .map(|(i, _)| format!("${}", i + 1))
            .collect::<Vec<String>>()
            .join(",");

        let params = all_ids
            .into_iter()
            .map(|x| &*x as &(dyn ToSql + Sync))
            .collect::<Vec<&(dyn ToSql + Sync)>>();
        let params_slice = params.as_slice();

        info!("delete flags: {:?}, {}", params_slice, phs);

        stmt.execute(
            &format!(
                "update nodes set delete_time = CURRENT_TIMESTAMP where id in ({})",
                phs
            ),
            params_slice,
        )
        .await
        .map(|_| ())
        .map_err(|e| anyhow::Error::new(e))?;

        self.move_nodes(&NodeMoveReq {
            id: req.id.clone(),
            parent_id: Some(NodeId::from(MAGIC_RECYCLE_BIN)),
            prev_sliding_id: None,
        })
        .await
        .map(|_| ())
    }

    async fn update_node_name(&self, req: &NodeRenameReq) -> anyhow::Result<u64> {
        let stmt = self.pool.get().await?;
        stmt.execute(
            "update nodes set name = $1 where id = $2",
            &[&req.name, &req.id],
        )
        .await
        .map_err(|e| anyhow::Error::new(e))
    }

    async fn query_nodes(&self, node_filter: &NodeFetchReq) -> anyhow::Result<Vec<Node>> {
        let stmt = self.pool.get().await?;

        let sql = node_filter.to_sql();
        let nodes = stmt
            .query(&sql, &[])
            .await
            .map(|rows| rows.iter().map(|row| Self::map_row_node(&row)).collect())?;

        Ok(nodes)
    }

    async fn move_nodes(&self, node_move_req: &NodeMoveReq) -> anyhow::Result<NodeMoveRsp> {
        let node_id = &node_move_req.id;
        let parent_id = &node_move_req.parent_id;
        let prev_slibing = &node_move_req.prev_sliding_id;

        info!("move {:?} to {:?}.{:?}|^", node_id, parent_id, prev_slibing);

        let old = self._delete_relation(node_id, false).await?;
        let new = self
            ._insert_relation(node_id, parent_id, prev_slibing)
            .await?;

        Ok(NodeMoveRsp { new, old })
    }

    async fn _delete_relation(
        &self,
        node_id: &NodeId,
        to_history: bool,
    ) -> anyhow::Result<NodeRelation> {
        let stmt = self.pool.get().await?;

        let rows = stmt
            .query(
                "select id, prev_sliding_id, parent_id from nodes where (id = $1 or prev_sliding_id = $2) and parent_id in (select parent_id from nodes where id = $3)", & [&node_id, &node_id, &node_id],
            )
            .await?;

        let mut parent_id = None::<NodeId>;
        let mut prev_id = None::<NodeId>;
        let mut next_id = None::<NodeId>;

        for row in rows {
            let id: String = row.get("id");
            let prev: String = row.get("prev_sliding_id");
            let parent: String = row.get("parent_id");
            if id.as_str() == node_id.as_str() {
                parent_id.replace(parent.into());
                prev_id.replace(prev.into());
            } else if prev.as_str() == node_id.as_str() {
                next_id.replace(id.into());
            }
        }

        if to_history {
            self._move_node_to_history(node_id).await?;
        }

        stmt.execute(
            "update nodes set prev_sliding_id = $1 where id = $2",
            &[&prev_id, &next_id],
        )
        .await?;

        Ok(NodeRelation {
            parent_id,
            prev_id,
            next_id,
        })
    }

    async fn _insert_relation(
        &self,
        node_id: &NodeId,
        new_parent_id: &Option<NodeId>,
        new_prev_id: &Option<NodeId>,
    ) -> anyhow::Result<NodeRelation> {
        let stmt = self.pool.get().await?;

        let rows = stmt
            .query(
                "select id, prev_sliding_id, parent_id from nodes where parent_id = $1 and prev_sliding_id = $2", & [&new_parent_id, &new_prev_id],
            )
            .await?;

        let mut next_id = None::<NodeId>;

        for row in rows {
            let id: String = row.get("id");
            let prev: String = row.get("prev_sliding_id");
            let parent: String = row.get("parent_id");
            if id.as_str() == prev.as_str() {
                next_id.replace(id.into());
            }
        }

        if stmt
            .execute(
                "update nodes set prev_sliding_id = $1, parent_id = $2 where id = $3",
                &[&new_prev_id, &new_parent_id, &node_id],
            )
            .await?
            <= 0
        {
            return anyhow::bail!("there are no node with id: {:?}", node_id);
        }

        if let Some(next) = next_id.as_ref() {
            stmt.execute(
                "update nodes set prev_sliding_id = $1 where id = $2",
                &[&node_id, &next],
            )
            .await?;
        }

        Ok(NodeRelation {
            parent_id: new_parent_id.clone(),
            prev_id: new_prev_id.clone(),
            next_id,
        })
    }

    async fn _move_node_to_history(&self, node_id: &NodeId) -> anyhow::Result<u64> {
        let stmt = self.pool.get().await?;

        Ok(stmt.execute(
            "WITH moved_rows AS (
DELETE FROM nodes a
where id = $1
RETURNING a.*
)
INSERT INTO nodes_history 
SELECT id, name, content, node_type, username, delete_time, version_time, initial_time FROM moved_rows;",
            &[&node_id]
        ).await?)
    }

    async fn find_descendant_ids(
        &self,
        id: &NodeId,
    ) -> anyhow::Result<HashMap<NodeId, Option<NodeId>>> {
        let stmt = self.pool.get().await?;

        let map = stmt
            .query(
                "with recursive children(id, parent_id) as (
select n.id, n.parent_id from nodes n where n.id = $1
union 
select n.id, n.parent_id from nodes n, children c where n.parent_id = c.id
)
select * from children;",
                &[&id],
            )
            .await?
            .iter()
            .map(|row| (row.get("id"), row.get("parent_id")))
            .collect();
        Ok(map)
    }

    async fn find_ancestor_ids(
        &self,
        id: &NodeId,
    ) -> anyhow::Result<HashMap<NodeId, Option<NodeId>>> {
        let stmt = self.pool.get().await?;

        let map = stmt
            .query(
                "with recursive children(id, parent_id) as (
select n.id, n.parent_id from nodes n where n.id = $1
union 
select n.id, n.parent_id from nodes n, children c where n.id = c.parent_id
)
select * from children;",
                &[&id],
            )
            .await?
            .iter()
            .map(|row| (row.get("id"), row.get("parent_id")))
            .collect();
        Ok(map)
    }
}

#[async_trait]
impl Mapper for PostgresMapper {
    async fn ensure_table_nodes(&self) -> anyhow::Result<()> {
        self.create_table(
            constants::TABLE_NAME_NODES,
            "CREATE TABLE nodes (
    id VARCHAR(40) NOT NULL,
    name VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    node_type VARCHAR(255) NOT NULL,
    username TEXT NOT NULL,
    delete_time timestamptz DEFAULT NULL,
    parent_id VARCHAR(40) DEFAULT NULL,
    prev_sliding_id VARCHAR(40),
    version_time timestamptz NOT NULL default CURRENT_TIMESTAMP,
    initial_time timestamptz NOT NULL,
    primary key (id)
);",
        )
        .await?;

        self.create_table(
            constants::TABLE_NAME_NODES_HISTORY,
            "CREATE TABLE nodes_history (
    id VARCHAR(40) NOT NULL,
    name VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    node_type VARCHAR(255) NOT NULL,
    username TEXT NOT NULL,
    delete_time timestamptz DEFAULT NULL,
    version_time timestamptz NOT NULL default CURRENT_TIMESTAMP,
    initial_time timestamptz NOT NULL
);",
        )
        .await?;

        let client = self.get_client().await?;
        client
            .execute(
                "CREATE INDEX if not exists idx_nodes_history_id ON nodes_history (id);",
                &[],
            )
            .await
            .map(|_| ())
            .map_err(anyhow::Error::new)
    }

    async fn ensure_table_tags(&self) -> anyhow::Result<()> {
        self.create_table(constants::TABLE_NAME_TAGS, "").await
    }

    async fn ensure_table_alarm_instances(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn ensure_table_alarm_definations(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn ensure_table_assets(&self) -> anyhow::Result<()> {
        self.create_table(
            constants::TABLE_NAME_ASSETS,
            "CREATE TABLE assets (
    id VARCHAR(40) NOT NULL,
    ori_file_name TEXT NOT NULL,
    username TEXT NOT NULL,
    create_time timestamptz NOT NULL default CURRENT_TIMESTAMP,
    content_type TEXT,
    primary key (id)
);",
        )
        .await
    }
}

impl<'a> tokio_postgres::types::FromSql<'a> for NodeId {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        <&str as tokio_postgres::types::FromSql>::from_sql(ty, raw).map(|o| {
            let s = o.to_string();
            NodeId::from(s)
        })
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        <&str as tokio_postgres::types::FromSql>::accepts(ty)
    }
}

impl tokio_postgres::types::ToSql for NodeId {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        let inner = self.as_str();
        <&str as ToSql>::to_sql(&inner, ty, out)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
    where
        Self: Sized,
    {
        <&str as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}
