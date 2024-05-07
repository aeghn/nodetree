use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::node::{NodeId, NodeType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Nodes {
    pub id: NodeId,

    #[serde(default)]
    pub delete_time: Option<DateTime<Utc>>,

    pub name: String,
    pub content: String,
    pub node_type: NodeType,

    pub username: String,

    #[serde(default)]
    pub parent_id: Option<NodeId>,

    #[serde(default)]
    pub prev_sliding_id: Option<NodeId>,

    pub version_time: DateTime<Utc>,
    pub initial_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodesHistory {
    pub id: NodeId,

    #[serde(default)]
    pub delete_time: Option<DateTime<Utc>>,

    pub name: String,
    pub content: String,
    pub node_type: NodeType,

    pub username: String,

    pub version_time: DateTime<Utc>,
    pub initial_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Assets {
    pub id: String,
    pub ori_file_name: String,
    pub username: String,
    pub create_time: DateTime<Utc>,
    pub content_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Table {
    Nodes(Nodes),
    NodesHistory(NodesHistory),
    Assets(Assets),
}
