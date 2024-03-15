use anyhow::Ok;
use async_trait::async_trait;
use bytes::BytesMut;
use deadpool_postgres::{Client, Pool};
use postgres_types::{to_sql_checked, ToSql};
use serde::Deserialize;
use tokio_postgres::Row;
use tracing::info;

use crate::{
    constants,
    model::{
        node::{Node, NodeId, NodeMapper},
        nodefilter::NodeFilter,
    },
};

use super::Mapper;

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
    pool: Pool,
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

    fn map_nodes_row(row: &Row) -> Node {
        let id = row.get("id");
        Node {
            id,
            version: row.get("version"),
            name: row.get("name"),
            content: row.get("content"),
            user: row.get("username"),
            todo_status: row.get("todo_status"),
            tags: vec![],
            parent_id: row.get("parent_id"),
            prev_sliding_id: row.get("prev_sliding_id"),
            create_date: row.get("create_date"),
            first_version_date: row.get("first_version_date"),
        }
    }
}

#[async_trait]
impl NodeMapper for PostgresMapper {
    async fn insert_node_simple(&self, node: &Node) -> anyhow::Result<()> {
        let stmt = self.pool.get().await?;
        stmt.execute("insert into nodes(id, version, name, content, username, parent_id, todo_status,
             prev_sliding_id, create_date, first_version_date) values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
        &[
            &node.id,
            &node.version,
            &node.name,
            &node.content,
            &node.user,
            &node.parent_id,
            &node.todo_status,
            &node.prev_sliding_id,
            &node.create_date,
            &node.first_version_date
        ])
        .await
        .map_err(|e| {anyhow::Error::new(e)})
        .map(|_| ())
    }

    async fn delete_node_by_id(&self, id: &NodeId) -> anyhow::Result<()> {
        let stmt = self.pool.get().await?;

        stmt.execute("delete from nodes where id = ?", &[&id])
            .await
            .map(|_| ())
            .map_err(|e| anyhow::Error::new(e))
    }

    async fn query_nodes(&self, node_filter: &NodeFilter) -> anyhow::Result<Vec<Node>> {
        match node_filter {
            NodeFilter::Descendants(id) => {
                let stmt = self.pool.get().await?;

                let nodes: Vec<Node> = stmt
                    .query("select * from nodes where id = ?", &[&id])
                    .await
                    .map(|rows| rows.iter().map(|row| Self::map_nodes_row(&row)).collect())?;

                Ok(nodes)
            }
            NodeFilter::All => {
                let stmt = self.pool.get().await?;

                let nodes: Vec<Node> = stmt
                    .query("select * from nodes", &[])
                    .await
                    .map(|rows| rows.iter().map(|row| Self::map_nodes_row(&row)).collect())?;

                Ok(nodes)
            }
            ot => Err(anyhow::anyhow!("unhandled node filter `{:?}'", ot)),
        }
    }

    async fn move_nodes(
        &self,
        node_id: &NodeId,
        parent_id: &NodeId,
        prev_slibing: &NodeId,
    ) -> anyhow::Result<()> {
        let stmt = self.pool.get().await?;

        let rows = stmt
            .query(
                "select * from nodes where id in $1 or prev_sliding_id == $2 or (parent_id = $3 and prev_node = $4)",
                &[&node_id, &node_id],
            )
            .await?;

        let mut node = None;
        let mut new_next = None;
        let mut old_next = None;

        for row in rows {
            let n = Self::map_nodes_row(&row);
            if &n.id == node_id {
                node.replace(n);
            } else if &n.prev_sliding_id == prev_slibing && &n.parent_id == parent_id {
                new_next.replace(n);
            } else if &n.prev_sliding_id == node_id {
                old_next.replace(n);
            }
        }

        let old_prev_id = &node.unwrap().prev_sliding_id;

        if let Some(old_next) = old_next {
            stmt.execute(
                "update prev_sliding_id = $1 where id = $2 and version = $3",
                &[old_prev_id, &old_next.id, &old_next.version],
            )
            .await?;
        }

        stmt.execute(
            "update prev_sliding_id = ? and parent_id = ? where id = ?",
            &[&prev_slibing, &parent_id, &node_id],
        )
        .await?;

        if let Some(new_next) = new_next {
            stmt.execute(
                "update prev_sliding_id = $1 where id = $2 and version = $3",
                &[node_id, &new_next.id, &new_next.version],
            )
            .await?;
        }

        Ok(())
    }
}

#[async_trait]
impl Mapper for PostgresMapper {
    async fn ensure_table_nodes(&self) -> anyhow::Result<()> {
        self.create_table(
            constants::TABLE_NAME_NODES,
            "CREATE TABLE nodes (
    id VARCHAR(40) NOT NULL,
    version SMALLINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    username TEXT NOT NULL,
    todo_status VARCHAR(50) DEFAULT NULL,
    parent_id VARCHAR(255) NOT NULL,
    prev_sliding_id VARCHAR(255),
    create_date VARCHAR(19) NOT NULL,
    first_version_date VARCHAR(19) NOT NULL,
    primary key (id, version)
);",
        )
        .await
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

/* impl tokio_postgres::types::ToSql for NodeFilter {
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
} */
