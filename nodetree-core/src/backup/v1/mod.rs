pub mod rowmapper;
pub mod table;

use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use chrono::{DateTime, Days, NaiveDateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};
use tracing::info;

use crate::{
    config::{BackupConfig, Config},
    mapper::postgres_mapper::PostgresMapper,
    parser::asset::asset_path_by_uuid,
};

use self::{rowmapper::row_to_table, table::TableRow};

use super::BackupVersion;

pub enum RowSum {
    Pg(tokio_postgres::Row),
}

#[derive(Serialize, Deserialize, Clone, Debug, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum BackupContent {
    Nodes,
    NodesHistory,
    Assets,
    AssetFiles,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Backup {
    version: BackupVersion,
    content_type: BackupContent,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    value: Vec<TableRow>,
}

#[async_trait]
pub trait BackupHandlerV1 {
    async fn fetch_table(
        &self,
        backup_type: &BackupContent,
        time_field: &str,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> anyhow::Result<Vec<TableRow>>;

    async fn _backup_table(
        &self,
        dir: &PathBuf,
        content_type: &BackupContent,
        time_field: &str,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> anyhow::Result<Vec<TableRow>> {
        let mut backup_file = dir.clone();
        let table = content_type.as_ref();
        backup_file.push(format!("{}.json", table));
        let vec: Vec<TableRow> = self
            .fetch_table(&content_type, &time_field, start_time, end_time)
            .await?;

        if vec.len() > 0 {
            let file = std::fs::File::create(backup_file.as_path())?;

            serde_json::to_writer(file, &vec)?;
        }

        info!("Finished backuping: {:?}", content_type);

        Ok(vec)
    }

    async fn backup_asset_files(
        &self,
        assets: &Vec<TableRow>,
        backup_dir: &PathBuf,
        config: &Config,
    ) -> anyhow::Result<u64> {
        let asset_backup_dir = backup_dir.join("assets");
        tokio::fs::create_dir_all(&asset_backup_dir).await?;
        let mut count = 0;
        for t in assets {
            if let TableRow::AssetsRow(asset) = t {
                let filepath = asset_path_by_uuid(config, &asset.id);

                if tokio::fs::try_exists(&filepath).await? {
                    tokio::fs::copy(filepath, asset_backup_dir.join(&asset.id)).await?;
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    async fn backup_time_to_time(
        &self,
        base_dir: &str,
        config: &Arc<Config>,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> anyhow::Result<()> {
        let backup_dir_name = format!(
            "nodetree-{}-{}",
            start_time.format("%Y%m%dT%H%M%S"),
            end_time.format("%Y%m%dT%H%M%S")
        );

        info!("Backup: {} -- {}", start_time, end_time);

        let backup_dir = PathBuf::from(base_dir).join(&backup_dir_name);

        tokio::fs::create_dir_all(backup_dir.as_path()).await?;
        let mut row_count = 0;

        row_count += self
            ._backup_table(
                &backup_dir,
                &BackupContent::Nodes,
                &"version_time",
                start_time,
                end_time,
            )
            .await?
            .len();

        row_count += self
            ._backup_table(
                &backup_dir,
                &BackupContent::NodesHistory,
                &"version_time",
                start_time,
                end_time,
            )
            .await?
            .len();

        let assets = self
            ._backup_table(
                &backup_dir,
                &BackupContent::Assets,
                &"create_time",
                start_time,
                end_time,
            )
            .await?;
        row_count += assets.len();

        if assets.len() > 0 {
            self.backup_asset_files(&assets, &backup_dir, &config)
                .await?;
        }

        if row_count > 0 {
            let backup_tar_path = PathBuf::from(base_dir).join(backup_dir_name.clone() + ".tar");
            info!("creating backup tar: {:?}", backup_tar_path);
            let tar_file = tokio::fs::File::create(backup_tar_path).await?;
            let mut tar_builder = tokio_tar::Builder::new(tar_file);

            tar_builder
                .append_dir_all("backup", backup_dir.as_path())
                .await?;
        }

        tokio::fs::remove_dir_all(backup_dir.as_path()).await?;

        Ok(())
    }

    async fn backup_all(&self, base_dir: &str, config: &Arc<Config>) -> anyhow::Result<()> {
        let start = DateTime::from_timestamp(0, 0).unwrap();
        let end = Utc::now();
        self.backup_time_to_time(base_dir, &config, &start, &end)
            .await
    }

    async fn backup_increasely(
        &self,
        back_config: &BackupConfig,
        config: &Arc<Config>,
    ) -> anyhow::Result<()> {
        let mut read_dir = tokio::fs::read_dir(back_config.dir.as_str()).await?;

        let mut start_time = None;

        while let Some(entry) = read_dir.next_entry().await? {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            let file_prefix = file_name.split(".").collect::<Vec<&str>>()[0];

            let parts: Vec<&str> = file_prefix.split("-").collect();

            let end = parts[2];

            let end: DateTime<Utc> = NaiveDateTime::parse_from_str(end, "%Y%m%dT%H%M%S")?.and_utc();

            if let Some(st) = start_time {
                if end > st {
                    start_time.replace(end);
                }
            } else {
                start_time.replace(end);
            }
        }

        match start_time {
            Some(st) => {
                let mut st = st.clone();
                let now = Utc::now();
                while now - st
                    > TimeDelta::try_seconds(back_config.interval.unwrap_or(3600) as i64).unwrap()
                {
                    let end_time = st.checked_add_days(Days::new(3)).unwrap();
                    let end_time = if now < end_time { now } else { end_time };

                    self.backup_time_to_time(&back_config.dir, &config, &st, &end_time)
                        .await?;

                    st = end_time;
                }
            }
            None => {
                if let Err(err) = self.backup_all(&back_config.dir, &config).await {
                    tracing::error!("Unable to backup all, {}", err);
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl BackupHandlerV1 for PostgresMapper {
    async fn fetch_table(
        &self,
        table_name: &BackupContent,
        time_field: &str,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> anyhow::Result<Vec<TableRow>> {
        let stmt = self.pool.get().await?;

        let vec = stmt
            .query(
                format!(
                    "select * from {} where {} <= $1 and {} >= $2",
                    table_name.as_ref(),
                    time_field,
                    time_field
                )
                .as_str(),
                &[&end_time, &start_time],
            )
            .await?;

        let mut arr: Vec<TableRow> = vec![];
        for ele in vec.into_iter() {
            match row_to_table(&table_name, RowSum::Pg(ele)) {
                Ok(r) => {
                    arr.push(r);
                }
                Err(err) => return Err(err),
            }
        }

        Ok(arr)
    }
}
