use async_trait::async_trait;

use crate::backup::v1::BackupHandlerV1;

use self::{asset::AssetMapper, node::NodeMapper, todo::TodoMapper};

#[cfg(feature = "postgres")]
pub mod postgres_mapper;

pub mod asset;
pub mod node;
pub mod nodefilter;
#[cfg(feature = "sqlite")]
pub mod sqlite_mapper;
pub mod todo;

#[async_trait]
pub trait Mapper: Sync + Send + NodeMapper + AssetMapper + BackupHandlerV1 + TodoMapper {
    async fn ensure_table_nodes(&self) -> anyhow::Result<()>;
    async fn ensure_table_tags(&self) -> anyhow::Result<()>;
    async fn ensure_table_todos(&self) -> anyhow::Result<()>;
    async fn ensure_table_alarm_instances(&self) -> anyhow::Result<()>;
    async fn ensure_table_alarm_definations(&self) -> anyhow::Result<()>;

    async fn ensure_table_assets(&self) -> anyhow::Result<()>;

    async fn get_table_fields(&self, table_name: &str) -> anyhow::Result<Vec<String>>;

    async fn init(&mut self) -> anyhow::Result<()>;

    async fn ensure_tables(&self) -> anyhow::Result<()> {
        self.ensure_table_nodes().await?;
        self.ensure_table_tags().await?;
        self.ensure_table_alarm_definations().await?;
        self.ensure_table_alarm_instances().await?;
        self.ensure_table_assets().await?;

        Ok(())
    }
}
