
use async_trait::async_trait;
use serde::{Deserialize, Serialize};


use super::{nodefilter::NodeFilter, tag::Tag};

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub version: i16,

    pub name: String,
    pub content: String,

    pub user: String,
    pub todo_status: Option<String>,
    pub tags: Vec<Tag>,

    pub parent_id: String,
    pub prev_sliding_id: Option<String>,

    pub create_date: String,
    pub first_version_date: String,
}

#[async_trait]
pub trait NodeMapper {
    async fn update_or_insert_node(&self, node: &Node) -> anyhow::Result<()>;
    async fn delete_node_by_id(&self, id: &str) -> anyhow::Result<()>;
    async fn query_nodes(&self, node_filter: NodeFilter) -> anyhow::Result<Vec<Node>>;

    async fn move_nodes(
        &self,
        node_id: &str,
        parent_id: &str,
        prev_slibing: Option<&str>,
    ) -> anyhow::Result<()>;
}
