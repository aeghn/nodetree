use async_trait::async_trait;

#[cfg(feature = "postgres")]
pub mod postgres_mapper;

#[cfg(feature = "sqlite")]
pub mod sqlite_mapper;

mod constants;

#[async_trait]
pub trait Mapper: Sync + Send {
    async fn ensure_table_nodes(&self) -> anyhow::Result<()>;
    async fn ensure_table_tags(&self) -> anyhow::Result<()>;
    async fn ensure_table_alarm_instances(&self) -> anyhow::Result<()>;
    async fn ensure_table_alarm_definations(&self) -> anyhow::Result<()>;

    async fn ensure_tables(&self) -> anyhow::Result<()> {
        self.ensure_table_nodes().await?;
        self.ensure_table_tags().await?;
        self.ensure_table_alarm_definations().await?;
        self.ensure_table_alarm_instances().await?;

        Ok(())
    }
}
