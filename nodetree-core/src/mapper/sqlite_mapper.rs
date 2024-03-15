use async_trait::async_trait;
use deadpool_sqlite::Pool;
use serde::Deserialize;

use crate::model::{
    node::{Node, NodeId, NodeMapper},
    nodefilter::NodeFilter,
};

use super::Mapper;

pub struct SqliteMapper {
    pool: Pool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SqliteConfig {
    filepath: String,
}

impl Into<deadpool_sqlite::Config> for SqliteConfig {
    fn into(self) -> deadpool_sqlite::Config {
        deadpool_sqlite::Config::new(self.filepath)
    }
}

impl SqliteMapper {
    pub fn new(config: SqliteConfig) -> anyhow::Result<SqliteMapper> {
        let config: deadpool_sqlite::Config = config.into();

        let pool = config.builder(deadpool_sqlite::Runtime::Tokio1)?.build()?;
        Ok(SqliteMapper { pool })
    }
}

#[async_trait]
impl Mapper for SqliteMapper {
    async fn ensure_table_nodes(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn ensure_table_tags(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn ensure_table_alarm_instances(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn ensure_table_alarm_definations(&self) -> anyhow::Result<()> {
        todo!()
    }
}

#[async_trait]
impl NodeMapper for SqliteMapper {
    async fn insert_node_simple(&self, node: &Node) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete_node_by_id(&self, id: &NodeId) -> anyhow::Result<()> {
        todo!()
    }

    async fn query_nodes(&self, node_filter: &NodeFilter) -> anyhow::Result<Vec<Node>> {
        todo!()
    }

    async fn move_nodes(
        &self,
        node_id: &NodeId,
        parent_id: &NodeId,
        prev_slibing: &NodeId,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
