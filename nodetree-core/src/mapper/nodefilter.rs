use std::{any::Any, vec};

use regex::Regex;
use serde::{de, Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

use crate::model::node::NodeId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeFetchReq {
    pub selection: Option<Vec<NodeSelection>>,
    pub filter: Option<NodeFilter>,
}

impl NodeFetchReq {
    fn with_selection(&self, selection: &NodeSelection, def: bool) -> bool {
        if self.selection.is_none() {
            def
        } else {
            self.selection
                .as_ref()
                .unwrap()
                .iter()
                .any(|e| e == selection)
        }
    }

    fn with_limit(&self) -> Option<i32> {
        if self.selection.is_none() {
        } else {
            for ele in self.selection.as_ref().unwrap() {
                match ele {
                    NodeSelection::Limit(c) => return Some(c.to_owned()),
                    _ => {}
                }
            }
        }
        None
    }

    pub fn to_sql(&self) -> String {
        let with_content = self.with_selection(&NodeSelection::WithContent, false);

        let with_history = self.with_selection(&NodeSelection::WithHistory, false);

        let with_limit = self.with_limit();

        let selection = if with_content {
            "*"
        } else {
            "n.id, n.delete_time,  n.name, n.username, n.node_type, n.parent_id, n.prev_sliding_id, n.create_time, n.first_version_time"
        };

        let mut s = format!("select {} from nodes n", selection);

        if let Some(f) = self.filter.as_ref() {
            let part = f.to_sql();
            if !part.is_empty() {
                s.push_str(" where ");
                s.push_str(&part);
            }
        }

        if let Some(limit) = with_limit {
            s.push_str(&format!(" limit {}", limit))
        }

        info!("Query sql is: {}", s);

        s
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum NodeSelection {
    WithContent,
    WithHistory,
    Limit(i32),
}

impl TryFrom<&str> for NodeSelection {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("lim") {
            let re = Regex::new(r"\d+$").unwrap();

            if let Some(captures) = re.captures(value) {
                if let Some(num_str) = captures.get(0) {
                    if let Ok(num) = num_str.as_str().parse::<i32>() {
                        return Ok(Self::Limit(num));
                    }
                }
            } else {
                return Ok(Self::Limit(6));
            }
        }

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
    Contains(String),
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
            "contains" | "like" => {
                let value = value.as_str().unwrap().into();
                Ok(NodeFilter::Contains(value))
            }
            key => Err(format!("NodeFilter: unknown filter: `{}'", key)),
        }
    }

    pub fn to_sql(&self) -> String {
        let inner = match &self {
            NodeFilter::All => "".to_string(),
            NodeFilter::Children(id) => {
                format!("n.parent_id = '{}'", id.as_str())
            }
            NodeFilter::Id(id) => {
                format!("n.id = '{}'", id.as_str())
            }
            NodeFilter::Tag(tag) => {
                format!(
                    "n.id in (select node_id from tags t where t.tag = '{}')",
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
            NodeFilter::Contains(part) => {
                format!("n.content like '%{}%' or n.name like '%{}%'", part, part)
            }
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
