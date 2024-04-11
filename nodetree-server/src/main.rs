use std::sync::Arc;

use arguments::Arguments;
use clap::Parser;
use config::Config;
use ntcore::mapper::Mapper;
use tracing::{info, Level};

mod arguments;
mod config;
mod controller;
pub mod utils;
mod service;

async fn write_default_config(filepath: &str) {
    tokio::fs::write(filepath, include_str!("../../data/config.example.toml"))
        .await
        .expect("unable to write default config")
}

#[tokio::main]
async fn main() {
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
            let config: Config = toml::from_str(cf.as_str()).unwrap();
            let mapper: anyhow::Result<Arc<dyn Mapper + 'static>> = config.db_config.clone().into();
            let mapper = mapper.unwrap();

            mapper.ensure_tables().await.unwrap();
            controller::serve(mapper, config).await;
        }
        Err(err) => {
            info!("unable to read err, creating default config to it. {}", err);
            write_default_config(args.config.as_str()).await;
        }
    }
}
