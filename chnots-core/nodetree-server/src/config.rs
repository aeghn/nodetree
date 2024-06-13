use std::ops::Deref;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(rename = "mapper")]
    #[serde(flatten)]
    pub config: kcore::config::Config,
    pub server: Server,
}

impl Deref for ServerConfig {
    type Target = kcore::config::Config;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

pub mod tests {
    #[test]
    fn test_db_deserialize() {
        let toml_str = r#"
        [db_config]
        type = "sqlite"
        filepath = "/home/123"
    "#;

        let config: super::ServerConfig = toml::from_str(toml_str).unwrap();
        println!("{:?}", config);
    }
}
