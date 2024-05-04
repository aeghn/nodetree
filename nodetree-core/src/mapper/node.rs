use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::model::node::{self, ContentParsedInfo, Node, NodeId};

use super::nodefilter::{NodeFetchReq, NodeFilter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeMoveRsp {
    pub new_parent: Option<NodeId>,
    pub new_prev: Option<NodeId>,
    pub new_next: Option<NodeId>,
    pub old_parent: Option<NodeId>,
    pub old_prev: Option<NodeId>,
    pub old_next: Option<NodeId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeMoveReq {
    pub id: NodeId,
    pub parent_id: Option<NodeId>,
    pub prev_sliding_id: Option<NodeId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NodeInsertResult {
    ParsedInfo(ContentParsedInfo),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeDeleteReq {
    pub id: NodeId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeRenameReq {
    pub id: NodeId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeUpdateContentReq {
    pub id: NodeId,
    pub content: String,
    pub version_time: DateTime<Utc>,
}

#[async_trait]
pub trait NodeMapper {
    async fn insert_and_move(&self, node: &Node) -> anyhow::Result<NodeInsertResult> {
        error!("aaaaaaaaaaaaaaa: {:?}", node);
        let insert_result = self.insert_node_only(node).await;
        let move_req = NodeMoveReq {
            id: node.id.clone(),
            parent_id: node.parent_id.clone(),
            prev_sliding_id: node.prev_sliding_id.clone(),
        };

        self.move_nodes(&move_req).await?;

        insert_result
    }

    /// Just insert a node into nodes table, do not care about nodes relations.  
    /// So do not use this method directly.
    async fn insert_node_only(&self, node: &Node) -> anyhow::Result<NodeInsertResult>;

    /// Delete node (logical or physical).
    /// 1. Delete node and its descentants.  
    ///    a. find all its descentants and mark them(both in nodes and node_history table)  
    ///    b. make its next slibing connect to its prev slibing.
    /// 2. Delete node but level its descentants(TODO).
    async fn delete_node(&self, req: &NodeDeleteReq) -> anyhow::Result<()>;
    async fn update_node_name(&self, req: &NodeRenameReq) -> anyhow::Result<u64>;
    async fn update_node_content(
        &self,
        req: &NodeUpdateContentReq,
    ) -> anyhow::Result<NodeInsertResult> {
        let node = self
            .query_nodes(&NodeFetchReq {
                selection: None,
                filter: Some(NodeFilter::Id(req.id.clone())),
            })
            .await;

        if let Ok(Some(node)) = node.as_ref().map(|e| e.get(0)) {
            self.insert_node_only(&Node {
                content: req.content.clone(),
                version_time: req.version_time,
                ..node.clone()
            })
            .await
        } else {
            anyhow::bail!("unable to save to db")
        }
    }

    async fn query_nodes(&self, node_filter: &NodeFetchReq) -> anyhow::Result<Vec<Node>>;

    /// Move Node.
    ///   
    /// ```
    /// * Node A           |    * Node A    
    ///   * Node D         |      * Node D  
    ///   * Node E         |      + Node X  
    /// * Node B           |       + Node Y  
    /// * Node C           |      * Node E  
    ///   - Node P         |    * Node B
    ///   - Node X         |    * Node C  
    ///     - Node Y       |      * Node P  
    ///   * Node F         |      * Node F  
    /// ```
    ///   
    /// 0. find X(self, record)
    /// 1. find `old_prev` P'id and `old_next` F(record)  
    /// 2. find the `prev_slibing` D's next `new_next` E(record)  
    /// 3. set F's prev as P  
    /// 4. set X's parent as A and X' prev as D and E's prev as X  
    async fn move_nodes(&self, node_move_req: &NodeMoveReq) -> anyhow::Result<NodeMoveRsp>;

    /// Find descendants recursively.  
    ///
    /// Return a HashMap which child_id points to its parent.
    async fn find_descendant_ids(
        &self,
        id: &NodeId,
    ) -> anyhow::Result<HashMap<NodeId, Option<NodeId>>>;

    /// Like `find_descendant_ids`, but find ancestors recursively.  
    ///
    /// Return a HashMap which child_id points to its parent.
    async fn find_ancestor_ids(
        &self,
        id: &NodeId,
    ) -> anyhow::Result<HashMap<NodeId, Option<NodeId>>>;
}
