use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::model::node::{self, ContentParsedInfo, Node, NodeId};

use super::nodefilter::NodeFilter;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeMoveRsp {
    pub new_parent: NodeId,
    pub new_prev: Option<NodeId>,
    pub new_next: Option<NodeId>,
    pub old_parent: NodeId,
    pub old_prev: Option<NodeId>,
    pub old_next: Option<NodeId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeMoveReq {
    pub id: NodeId,
    pub parent_id: NodeId,
    pub prev_sliding_id: Option<NodeId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NodeInsertResult {
    TooLittleChange,
    ParsedInfo(ContentParsedInfo),
}

#[async_trait]
pub trait NodeMapper {
    async fn insert_and_move(&self, node: &Node) -> anyhow::Result<NodeInsertResult> {
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

    async fn delete_node_by_id(&self, id: &NodeId) -> anyhow::Result<()>;
    async fn query_nodes(&self, node_filter: &NodeFilter) -> anyhow::Result<Vec<Node>>;

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
    async fn find_descendant_ids(&self, id: &NodeId) -> anyhow::Result<HashMap<NodeId, NodeId>>;

    /// Like `find_descendant_ids`, but find ancestors recursively.  
    ///
    /// Return a HashMap which child_id points to its parent.
    async fn find_ancestor_ids(&self, id: &NodeId) -> anyhow::Result<HashMap<NodeId, NodeId>>;
}
