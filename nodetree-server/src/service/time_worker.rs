use std::{path::PathBuf, sync::Arc};

use ntcore::{
    config::{BackupConfig, Config},
    mapper::Mapper,
};
use tokio::time;
use tracing::{error, info};

pub async fn backup(
    mapper: &Arc<dyn Mapper + 'static>,
    config: &Arc<Config>,
    backup_config: &BackupConfig,
) -> anyhow::Result<()> {
    let mapper = mapper.clone();
    let backup_config = backup_config.clone();
    let config = config.clone();
    tokio::spawn(async move {
        let mapper = mapper.clone();

        let mut interval = time::interval(time::Duration::from_secs(
            backup_config.interval.unwrap_or(86400) as u64,
        ));

        let backup_dir = PathBuf::from(backup_config.dir.clone());
        if let Err(err) = tokio::fs::create_dir_all(backup_dir.as_path()).await {
            error!("Unable to backup, {}", err);
            return;
        }

        loop {
            interval.tick().await;
            info!("begin to backup db");
            if let Err(err) = mapper.backup_increasely(&backup_config, &config).await {
                error!("Unable to backup increasely, {}", err);
            }
        }
    });
    Ok(())
}
