use std::sync::Arc;

use arguments::Arguments;
use clap::Parser;
use config::Config;
use ntcore::mapper::Mapper;
use tracing::Level;

mod arguments;
mod config;
mod controller;
mod service;
pub mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::time())
        .init();

    let args = Arguments::parse();

    let config: Config = toml::from_str(
        tokio::fs::read_to_string(args.config.as_str())
            .await
            .unwrap()
            .as_str(),
    )
    .unwrap();

    let mapper: anyhow::Result<Arc<dyn Mapper + 'static>> = config.db_config.clone().into();
    let mapper = mapper.unwrap();

    mapper.ensure_tables().await.unwrap();
    controller::serve(mapper, config).await;
}
