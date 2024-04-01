use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::constants::MAGIC_PREV_NODE_ID_EMPTY;

use super::tag::Tag;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: NodeId,

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
