use std::sync::Arc;

use arguments::Arguments;
use clap::Parser;
use config::Config;
use ntcore::mapper::Mapper;
use once_cell::sync::Lazy;
use tracing::Level;

mod arguments;
mod config;
mod controller;
mod service;

static ARGUMENTS: Lazy<Arguments> = Lazy::new(Arguments::parse);
static CONFIG: Lazy<Config> = Lazy::new(|| {
    toml::from_str(
        std::fs::read_to_string(ARGUMENTS.config.as_str())
            .unwrap()
            .as_str(),
    )
    .unwrap()
});

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_thread_ids(true)
        .with_timer(tracing_subscriber::fmt::time::time())
        .init();

    let mapper: anyhow::Result<Arc<dyn Mapper + 'static>> = CONFIG.db_config.clone().into();
    let mapper = mapper.unwrap();

    mapper.ensure_tables().await.unwrap();
    controller::serve(mapper, "0.0.0.0", &CONFIG.server.port).await;
}
