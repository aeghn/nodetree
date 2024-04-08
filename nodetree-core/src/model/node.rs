use std::fmt::Display;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};


use super::tag::Tag;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: NodeId,

    #[serde(default)]
    pub delete_time: Option<DateTime<Utc>>,

    pub name: String,
    pub content: String,

    pub user: String,
    pub parsed_info: ContentParsedInfo,

    #[serde(default)]
    pub parent_id: Option<NodeId>,

    #[serde(default)]
    pub prev_sliding_id: Option<NodeId>,

    pub create_time: DateTime<Utc>,
    pub first_version_time: DateTime<Utc>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]

pub struct ContentParsedInfo {
    #[serde(default)]
    pub todo_status: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc};
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
        let now_utc: DateTime<Utc> = Utc::now().to_owned();
        let v = serde_json::to_string(&now_utc).unwrap();

        let s = "\"2024-04-03T05:04:38.675Z\"".to_owned();
        let t = &v;
        println!("{}, {}", s, t);
        let v: DateTime<Utc> = serde_json::from_str(&s).unwrap();
        println!("{}", v)
    }
}
