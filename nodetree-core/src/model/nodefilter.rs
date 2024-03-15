use serde::Deserialize;

#[derive(Clone, Debug)]
pub enum NodeFilter {
    All,
    Descendants(String),
    Acendant(String),
    Id(String),
    Tag(String),
    And(Box<Vec<NodeFilter>>),
    Not(Box<NodeFilter>),
    Or(Box<Vec<NodeFilter>>),
    WithContent(Box<NodeFilter>),
}
