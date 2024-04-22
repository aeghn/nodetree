use async_trait::async_trait;

use self::{asset::AssetMapper, node::NodeMapper};

#[cfg(feature = "postgres")]
pub mod postgres_mapper;

pub mod asset;
pub mod node;
pub mod nodefilter;
pub mod operation;
#[cfg(feature = "sqlite")]
pub mod sqlite_mapper;

#[async_trait]
pub trait Mapper: Sync + Send + NodeMapper + AssetMapper {
    async fn ensure_table_nodes(&self) -> anyhow::Result<()>;
    async fn ensure_table_tags(&self) -> anyhow::Result<()>;
    async fn ensure_table_alarm_instances(&self) -> anyhow::Result<()>;
    async fn ensure_table_alarm_definations(&self) -> anyhow::Result<()>;

    async fn ensure_table_assets(&self) -> anyhow::Result<()>;

    async fn ensure_tables(&self) -> anyhow::Result<()> {
        self.ensure_table_nodes().await?;
        self.ensure_table_tags().await?;
        self.ensure_table_alarm_definations().await?;
        self.ensure_table_alarm_instances().await?;
        self.ensure_table_assets().await?;

        Ok(())
    }
}
