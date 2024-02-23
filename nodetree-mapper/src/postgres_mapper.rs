use anyhow::Ok;
use async_trait::async_trait;
use deadpool_postgres::{Client, Pool};
use serde::Deserialize;
use tracing::info;

use crate::{constants, Mapper};

#[derive(Debug, Deserialize)]
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
impl Mapper for PostgresMapper {
    async fn ensure_table_nodes(&self) -> anyhow::Result<()> {
        self.create_table(
            constants::TABLE_NAME_NODES,
            "CREATE TABLE nodes (
    id VARCHAR(40) PRIMARY KEY,
    version INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    is_private BOOLEAN NOT NULL,
    todo_status VARCHAR(50) NOT NULL,
    tags VARCHAR(255)[] NOT NULL,
    parent_id VARCHAR(255) NOT NULL,
    prev_sliding_id VARCHAR(255),
    create_date BIGINT NOT NULL,
    first_version_date BIGINT NOT NULL
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
