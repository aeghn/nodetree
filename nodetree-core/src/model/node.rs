use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::constants::MAGIC_PREV_NODE_ID_EMPTY;

use super::{nodefilter::NodeFilter, tag::Tag};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub version: i16,

    pub is_current: bool,
    pub delete_time: Option<NaiveDateTime>,

    pub name: String,
    pub content: String,

    pub user: String,
    pub todo_status: Option<String>,
    pub tags: Vec<Tag>,

    pub parent_id: NodeId,
    pub prev_sliding_id: Option<NodeId>,

    pub create_time: NaiveDateTime,
    pub first_version_time: NaiveDateTime,
}

#[async_trait]
pub trait NodeMapper {
    async fn insert_and_move(&self, node: &Node) -> anyhow::Result<NodeMoveResult> {
        self.insert_node_simple(node).await?;
        self.move_nodes(&node.id, &node.parent_id, node.prev_sliding_id.as_ref())
            .await
    }

    /// Just insert a node into nodes table, do not care about nodes relations.  
    /// So do not use this method directly.
    async fn insert_node_simple(&self, node: &Node) -> anyhow::Result<()>;

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
    async fn move_nodes(
        &self,
        node_id: &NodeId,
        parent_id: &NodeId,
        prev_slibing: Option<&NodeId>,
    ) -> anyhow::Result<NodeMoveResult>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeId(String);

impl NodeId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for NodeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for NodeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let o: String = String::deserialize(deserializer)?;
        Ok(NodeId(o))
    }
}

impl From<String> for NodeId {
    fn from(value: String) -> Self {
        Self { 0: value }
    }
}

impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        Self {
            0: value.to_string(),
        }
    }
}

impl From<Option<String>> for NodeId {
    fn from(value: Option<String>) -> Self {
        let inner = match value {
            Some(s) => s,
            None => MAGIC_PREV_NODE_ID_EMPTY.to_string(),
        };
        Self { 0: inner }
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use super::NodeId;

    #[derive(Deserialize, Serialize, Debug)]
    struct TestNodeId {
        id: NodeId,
    }

    #[test]
    fn test() {
        let n = TestNodeId { id: "".into() };
        let json_string = serde_json::to_string(&n).unwrap();
        println!("{}", json_string);
        let n: TestNodeId = serde_json::from_str(&json_string).unwrap();
        println!("{:?}", n);

        let json_string = "{\"id\": null}";
        let n: TestNodeId = serde_json::from_str(&json_string).unwrap();
        println!("{:?}", n)
    }

    #[test]
    fn time() {
        let now_utc = chrono::NaiveDateTime::from_timestamp(1, 1);
        let v = serde_json::to_string(&now_utc).unwrap();
        println!("{}", v);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeMoveResult {
    pub new_parent: NodeId,
    pub new_prev: Option<NodeId>,
    pub new_next: Option<NodeId>,
    pub old_parent: NodeId,
    pub old_prev: Option<NodeId>,
    pub old_next: Option<NodeId>,
}
