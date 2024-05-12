use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    log_and_bail,
    model::node::{self, ContentParsedInfo, MagicNodeId, Node, NodeId},
};

use super::nodefilter::{NodeFetchReq, NodeFilter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeMoveRsp {
    pub old: NodeRelation,
    pub new: NodeRelation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeRelation {
    pub parent_id: MagicNodeId,
    pub prev_id: MagicNodeId,
    pub next_id: MagicNodeId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeMoveReq {
    pub id: NodeId,
    pub parent_id: MagicNodeId,
    pub prev_sliding_id: MagicNodeId,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeUpdateReadonlyReq {
    pub id: NodeId,
    pub readonly: bool,
}

#[async_trait]
pub trait NodeMapper {
    async fn insert_and_move(&self, node: &Node) -> anyhow::Result<NodeInsertResult> {
        let insert_result = self
            .insert_node_only(&Node {
                parent_id: MagicNodeId::Never,
                prev_sliding_id: MagicNodeId::Never,
                ..node.clone()
            })
            .await?;

        self._insert_relation(&node.id, &node.parent_id, &node.prev_sliding_id)
            .await?;

        Ok(insert_result)
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

        match node.as_ref().map(|e| e.get(0)) {
            Ok(Some(node)) => {
                if !node.readonly {
                    self.insert_node_only(&Node {
                        content: req.content.clone(),
                        version_time: req.version_time,
                        ..node.clone()
                    })
                    .await
                } else {
                    log_and_bail!("node is readonly, {:?}", node)
                }
            }
            Ok(None) => {
                error!("Unable to fetch node, {:?}", req);
                anyhow::bail!("Unable to fetch node, {:?}", req)
            }
            Err(err) => {
                error!("Unable to fetch node, err {:?}", req);
                anyhow::bail!("Unable to fetch node, {:?}, err {:?}", req, err)
            }
        }
    }

    async fn update_node_readonly(&self, req: &NodeUpdateReadonlyReq) -> anyhow::Result<u64>;

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

    async fn _delete_relation(
        &self,
        node_id: &NodeId,
        to_history: bool,
    ) -> anyhow::Result<NodeRelation>;
    async fn _insert_relation(
        &self,
        node_id: &NodeId,
        parent_id: &MagicNodeId,
        prev_id: &MagicNodeId,
    ) -> anyhow::Result<NodeRelation>;

    async fn _move_node_to_history(&self, node_id: &NodeId) -> anyhow::Result<u64>;

    /// Find descendants recursively.  
    ///
    /// Return a HashMap which child_id points to its parent.
    async fn find_descendant_ids(
        &self,
        id: &NodeId,
    ) -> anyhow::Result<HashMap<NodeId, MagicNodeId>>;

    /// Like `find_descendant_ids`, but find ancestors recursively.  
    ///
    /// Return a HashMap which child_id points to its parent.
    async fn find_ancestor_ids(&self, id: &NodeId) -> anyhow::Result<HashMap<NodeId, MagicNodeId>>;
}
