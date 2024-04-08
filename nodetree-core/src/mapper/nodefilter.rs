use std::any::Any;

use serde::{de, Deserialize, Serialize};
use serde_json::Value;

use crate::model::node::NodeId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeFetchReq {
    selection: Vec<NodeSelection>,
    filter: Option<NodeFilter>,
}

impl NodeFetchReq {
    pub fn to_sql(&self) -> String {
        let with_content = self
            .selection
            .iter()
            .any(|e| e == &NodeSelection::WithContent);

        let with_history = self
            .selection
            .iter()
            .any(|e| e == &NodeSelection::WithHistory);

        let selection = if with_content {
            "*"
        } else {
            "id,delete_time, name, user, parent_id, prev_sliding_id, create_time, first_version_time"
        };

        let mut s = format!("select {} from nodes ", selection);

        if let Some(f) = self.filter.as_ref() {
            let part = f.to_sql();
            if !part.is_empty() {
                s.push_str(" where ");
                s.push_str(&part);
            }
        }

        s
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum NodeSelection {
    WithContent,
    WithHistory,
}

impl TryFrom<&str> for NodeSelection {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pairs = vec![
            (
                vec!["cont", "content", "with_content", "withcontent"],
                Self::WithContent,
            ),
            (
                vec!["his", "hist", "history", "with_history", "withhistory"],
                Self::WithHistory,
            ),
        ];

        for (cond, v) in pairs {
            if cond.contains(&value) {
                return Ok(v);
            }
        }

        Err(format!("Unknown key: {}", value))
    }
}

impl<'de> Deserialize<'de> for NodeSelection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        Self::try_from(value.as_str().unwrap()).map_err(|err| de::Error::custom(err))
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum NodeFilter {
    All,
    Children(NodeId),
    Id(NodeId),
    Tag(String),
    And(Box<Vec<NodeFilter>>),
    Not(Box<NodeFilter>),
    Or(Box<Vec<NodeFilter>>),
}

impl NodeFilter {
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
            "children" => {
                let value = value.as_str().unwrap().into();
                Ok(NodeFilter::Children(value))
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
                    .map(|e| Self::from_json(e).unwrap())
                    .collect();
                match filter {
                    "and" => Ok(NodeFilter::And(Box::new(vec))),
                    _ => Ok(NodeFilter::Or(Box::new(vec))),
                }
            }
            key => Err(format!("NodeFilter: unknown filter: `{}'", key)),
        }
    }

    pub fn to_sql(&self) -> String {
        let inner = match &self {
            NodeFilter::All => "".to_string(),
            NodeFilter::Children(id) => {
                format!("parent_id = '{}'", id.as_str())
            }
            NodeFilter::Id(id) => {
                format!("id = '{}'", id.as_str())
            }
            NodeFilter::Tag(tag) => {
                format!(
                    "id in (select node_id from tags where tag = '{}')",
                    tag.as_str()
                )
            }
            NodeFilter::And(nf) => nf
                .iter()
                .map(|e| e.to_sql())
                .filter(|e| !e.is_empty())
                .collect::<Vec<String>>()
                .join(" and "),
            NodeFilter::Not(nf) => {
                let p = nf.to_sql();
                if p.is_empty() {
                    format!("not {}", p)
                } else {
                    "".to_string()
                }
            }
            NodeFilter::Or(nf) => nf
                .iter()
                .map(|e| e.to_sql())
                .filter(|e| !e.is_empty())
                .collect::<Vec<String>>()
                .join(" or "),
        };

        return if !inner.is_empty() && !(inner.starts_with("(") && inner.ends_with(")")) {
            format!("({})", inner)
        } else {
            inner
        };
    }
}

impl<'de> Deserialize<'de> for NodeFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        Self::from_json(&value)
            .map(|f| f)
            .map_err(|err| de::Error::custom(err))
    }
}

#[cfg(test)]
mod test {
    use crate::mapper::nodefilter::NodeFilter;

    use super::NodeFetchReq;

    #[test]
    fn test() {
        let s = r#"
        {
            "filter": "and",
            "value": [
                {"filter": "all"},
                {"filter": "children",
                "value": "asdasd"}
            ]
        }
        "#;

        let j: Result<NodeFilter, serde_json::Error> = serde_json::from_str(s);
        println!("{:?}", j);

        let s = r#"
        {
            "selection": ["cont"],
        "filter":{
            "filter": "and",
            "value": [
                {"filter": "all"},
                {"filter": "children",
                "value": "asdasd"},
                {"filter": "and", "value": []}
            ]
        }
        }
        "#;

        let j: Result<NodeFetchReq, serde_json::Error> = serde_json::from_str(s);
        println!("{:?}, {:?}", &j, &j.as_ref().unwrap().to_sql());
    }
}
