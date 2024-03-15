use std::any::Any;

use serde::{de, Deserialize, Serialize};
use serde_json::Value;

use super::node::{Node, NodeId};

#[derive(Clone, Debug, Serialize)]
pub enum NodeFilter {
    All,
    Descendants(NodeId),
    Acendant(NodeId),
    Id(NodeId),
    Tag(String),
    And(Box<Vec<NodeFilter>>),
    Not(Box<NodeFilter>),
    Or(Box<Vec<NodeFilter>>),
    WithContent(Box<NodeFilter>),
}

fn from_json(jn: &Value) -> Result<NodeFilter, String> {
    let filter = match jn.get("filter") {
        Some(Value::String(v)) => Ok(v),
        Some(v) => Err(format!(
            "filter should be String, but it is {:?}",
            v.type_id(),
        )),
        None => Err("filter is missing".to_string()),
    }?;

    let value = match jn.get("value") {
        Some(v) => v,
        None => &Value::Null,
    };

    let filter = filter.to_ascii_lowercase();
    let filter = filter.as_str();

    match filter {
        "all" => Ok(NodeFilter::All),
        "descendants" => {
            let value = value.as_str().unwrap().into();
            Ok(NodeFilter::Descendants(value))
        }
        "acendant" => {
            let value = value.as_str().unwrap().into();
            Ok(NodeFilter::Acendant(value))
        }
        "id" => {
            let value = value.as_str().unwrap().into();
            Ok(NodeFilter::Id(value))
        }
        "and" | "or" => {
            let vec = value
                .as_array()
                .unwrap()
                .iter()
                .map(|e| from_json(e).unwrap())
                .collect();
            match filter {
                "and" => Ok(NodeFilter::And(Box::new(vec))),
                _ => Ok(NodeFilter::Or(Box::new(vec))),
            }
        }
        "withcontent" => {
            let v = from_json(value).unwrap();
            Ok(NodeFilter::WithContent(Box::new(v)))
        }
        key => Err(format!("NodeFilter: unknown filter: `{}'", key)),
    }
}

impl<'de> Deserialize<'de> for NodeFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        from_json(&value)
            .map(|f| f)
            .map_err(|err| de::Error::custom(err))
    }
}

#[cfg(test)]
mod test {
    use crate::model::{node::Node, nodefilter::NodeFilter};

    #[test]
    fn test() {
        let s = r#"
        {
            "filter": "and",
            "value": [
                {"filter": "all"},
                {"filter": "acendant",
                "value": "asdasd"}
            ]
        }
        "#;

        let j: Result<NodeFilter, serde_json::Error> = serde_json::from_str(s);
        println!("{:?}", j);
    }
}
