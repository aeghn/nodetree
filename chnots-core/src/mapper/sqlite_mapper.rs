use deadpool_sqlite::Pool;
use serde::Deserialize;

#[allow(dead_code)]
pub struct SqliteMapper {
    pool: Pool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SqliteConfig {
    filepath: String,
}

impl Into<deadpool_sqlite::Config> for SqliteConfig {
    fn into(self) -> deadpool_sqlite::Config {
        deadpool_sqlite::Config::new(self.filepath)
    }
}

impl SqliteMapper {
    pub fn new(config: SqliteConfig) -> anyhow::Result<SqliteMapper> {
        let config: deadpool_sqlite::Config = config.into();

        let pool = config.builder(deadpool_sqlite::Runtime::Tokio1)?.build()?;
        Ok(SqliteMapper { pool })
    }
}
