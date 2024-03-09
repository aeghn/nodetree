use anyhow::Ok;
use async_trait::async_trait;
use deadpool_postgres::{Client, Pool};
use ntcore::node::{Node, NodeMapper};
use serde::Deserialize;
use tracing::info;

use crate::{constants, Mapper};

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
}

#[async_trait]
impl NodeMapper for PostgresMapper {
    async fn update_or_insert_node(&self, node: &ntcore::node::Node) -> anyhow::Result<()> {
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

    async fn delete_node_by_id(&self, id: &str) -> anyhow::Result<()> {
        let stmt = self.pool.get().await?;

        stmt.execute("delete from nodes where id = ?", &[&id])
            .await
            .map(|_| ())
            .map_err(|e| anyhow::Error::new(e))
    }

    async fn query_nodes(
        &self,
        node_filter: ntcore::node::NodeFilter,
    ) -> anyhow::Result<Vec<ntcore::node::Node>> {
        match node_filter {
            ntcore::node::NodeFilter::FromParent(id) => {
                let stmt = self.pool.get().await?;

                let nodes: Vec<Node> = stmt
                    .query("select * from nodes where id = ?", &[&id])
                    .await
                    .map(|rows| {
                        rows.iter()
                            .map(|row| Node {
                                id: id.to_string(),
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
                            })
                            .collect()
                    })?;

                Ok(nodes)
            }
            ot => Err(anyhow::anyhow!("unhandled node filter {:?}", ot)),
        }
    }

    async fn move_nodes(
        &self,
        node_id: &str,
        parent_id: &str,
        prev_slibing: Option<&str>,
    ) -> anyhow::Result<()> {
        let stmt = self.pool.get().await?;

        let rows = stmt
            .query(
                "select * from nodes where id = ? or prev_sliding_id = ?",
                &[&node_id, &node_id],
            )
            .await?;

        let old_next: Vec<String> = rows
            .iter()
            .filter_map(|e| {
                if e.get::<&str, String>("prev_sliding_id") == node_id {
                    Some(e.get::<&str, String>("id"))
                } else {
                    None
                }
            })
            .collect();

        let old_row: Vec<(String, String)> = rows
            .iter()
            .filter(|e| e.get::<&str, String>("id") == node_id)
            .map(|e| (e.get("parent_id"), e.get("prev_sliding_id")))
            .collect();

        let old_parent: String = old_row[0].0.to_string();
        let old_prev: String = old_row[0].1.to_string();

        if let Some(next) = old_next.get(0) {
            stmt.execute(
                "update prev_sliding_id = ? where id = ?",
                &[&old_prev, next],
            )
            .await?;
        }

        stmt.execute(
            "update prev_sliding_id = ? and parent_id = ? where id = ?",
            &[&prev_slibing, &parent_id, &node_id],
        )
        .await?;

        Ok(())
    }
}

#[async_trait]
impl Mapper for PostgresMapper {
    async fn ensure_table_nodes(&self) -> anyhow::Result<()> {
        self.create_table(
            constants::TABLE_NAME_NODES,
            "CREATE TABLE nodes (
    id VARCHAR(40) PRIMARY KEY,
    version SMALLINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    username TEXT NOT NULL,
    todo_status VARCHAR(50) DEFAULT NULL,
    parent_id VARCHAR(255) NOT NULL,
    prev_sliding_id VARCHAR(255),
    create_date VARCHAR(19) NOT NULL,
    first_version_date VARCHAR(19) NOT NULL
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
