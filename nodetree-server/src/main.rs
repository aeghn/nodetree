use std::sync::Arc;

use arguments::Arguments;
use clap::Parser;
use config::ServerConfig;
use ntcore::mapper::Mapper;
use service::time_worker::backup;
use tracing::{info, Level};

pub mod adapter;
mod arguments;
mod config;
mod controller;
mod service;
pub mod utils;

async fn write_default_config(filepath: &str) {
    tokio::fs::write(filepath, include_str!("../../data/config.example.toml"))
        .await
        .expect("unable to write default config")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::time())
        .init();

    let args = Arguments::parse();

    let config_file = tokio::fs::read_to_string(args.config.as_str()).await;
    match config_file {
        Ok(cf) => {
            let config: ServerConfig = toml::from_str(cf.as_str())?;
            let mapper: anyhow::Result<Arc<dyn Mapper + 'static>> = config.db_config.clone().into().await;
            let mapper = mapper?;

            if let Some(backup_config) = config.backup.as_ref() {
                backup(&mapper, &Arc::new(config.config.clone()), backup_config).await?;
            }

            controller::serve(mapper, config).await;
        }
        Err(err) => {
            info!("unable to read err, creating default config to it. {}", err);
            write_default_config(args.config.as_str()).await;
        }
    }

    Ok(())
}
