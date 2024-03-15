use async_trait::async_trait;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::constants::MAGIC_PREV_NODE_ID_EMPTY;

use super::{nodefilter::NodeFilter, tag::Tag};

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub version: i16,

    pub name: String,
    pub content: String,

    pub user: String,
    pub todo_status: Option<String>,
    pub tags: Vec<Tag>,

    pub parent_id: NodeId,
    pub prev_sliding_id: NodeId,

    pub create_date: String,
    pub first_version_date: String,
}

#[async_trait]
pub trait NodeMapper {
    async fn insert_and_move(&self, node: &Node) -> anyhow::Result<()> {
        self.insert_node_simple(node).await?;
        self.move_nodes(&node.id, &node.parent_id, &node.prev_sliding_id)
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
        prev_slibing: &NodeId,
    ) -> anyhow::Result<()>;
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
        let o: Option<String> = Option::deserialize(deserializer)?;
        let inner = match o {
            Some(s) => s,
            None => MAGIC_PREV_NODE_ID_EMPTY.to_owned(),
        };

        Ok(NodeId(inner))
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
}
