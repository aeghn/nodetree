use std::{path::PathBuf, str::FromStr};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;

use super::BackupContent;

#[async_trait]
pub trait BackupHandlerV1 {
    async fn fetch_table(
        &self,
        table_name: &str,
        time_field: &str,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> anyhow::Result<Value>;

    async fn _backup_table(
        &self,
        dir: &PathBuf,
        content_type: &BackupContent,
        time_field: &str,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> anyhow::Result<()> {
        let mut backup_file = dir.clone();
        let table = content_type.as_ref();
        backup_file.push(format!("{}.json", table));
        let vec = self
            .fetch_table(table, &time_field, start_time, end_time)
            .await?;

        let file = std::fs::File::create(backup_file.as_path())?;

        serde_json::to_writer(file, &vec)?;

        Ok(())
    }

    async fn backup_time_to_time(
        &self,
        base_dir: &str,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> anyhow::Result<()> {
        let backup_dir_name = format!(
            "nodetree-{}-{}",
            start_time.format("%Y%m%d%H%M%S"),
            end_time.format("%Y%m%d%H%M%S")
        );

        let mut backup_dir = PathBuf::from(base_dir);
        backup_dir.push(backup_dir_name);

        tokio::fs::create_dir_all(backup_dir.as_path()).await?;

        let tables = [
            (BackupContent::Nodes, "version_time"),
            (BackupContent::NodesHistory, "version_time"),
            (BackupContent::Asset, "create_time"),
        ];
        for (table, field) in tables {
            self._backup_table(&backup_dir, &table, &field, start_time, end_time)
                .await?;
        }

        Ok(())
    }

    async fn backup_all(&self, base_dir: &str) -> anyhow::Result<()> {
        self.backup_all(base_dir).await
    }

    async fn backup_increasely(&self, base_dir: &str) -> anyhow::Result<()> {
        let mut read_dir = tokio::fs::read_dir(base_dir).await?;

        let mut start_time = DateTime::from_timestamp(0, 0).unwrap();

        while let Some(entry) = read_dir.next_entry().await? {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();

            let parts: Vec<&str> = file_name.split("-").collect();

            let end: DateTime<Utc> = DateTime::parse_from_str(parts[2], "%Y%m%d%H%M%S")?.into();

            if end > start_time {
                start_time = end;
            }
        }

        while start_time < Utc::now() {
            let end_time = Utc::now();
            let end_time = if start_time < end_time {
                start_time
            } else {
                end_time
            };

            self.backup_time_to_time(base_dir, &start_time, &end_time)
                .await?;

            start_time = end_time;
        }

        Ok(())
    }
}
