use std::sync::Arc;

use anyhow::Ok;
use ntcore::mapper::{
    postgres_mapper::{PostgresConfig, PostgresMapper},
    sqlite_mapper::{SqliteConfig, SqliteMapper},
    Mapper,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DbConfig {
    #[serde(rename = "postgres")]
    Postgres(PostgresConfig),
    #[serde(rename = "sqlite")]
    Sqlite(SqliteConfig),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Common {
    pub asset_base_dir: String,
}

impl Into<anyhow::Result<Arc<dyn Mapper>>> for DbConfig {
    fn into(self) -> anyhow::Result<Arc<(dyn Mapper + 'static)>> {
        let mapper = match self {
            DbConfig::Postgres(pg) => Arc::new(PostgresMapper::new(pg)?) as Arc<dyn Mapper>,
            /*             DbConfig::Sqlite(cfg) => Arc::new(SqliteMapper::new(cfg)?) as Arc<dyn Mapper>,
             */
            _ => todo!(),
        };

        Ok(mapper)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "mapper")]
    pub db_config: DbConfig,
    pub server: Server,
    pub common: Common,
}

pub mod tests {
    #[test]
    fn test_db_deserialize() {
        let toml_str = r#"
        [db_config]
        type = "sqlite"
        filepath = "/home/123"
    "#;

        let config: super::Config = toml::from_str(toml_str).unwrap();
        println!("{:?}", config);
    }
}
