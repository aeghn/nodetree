use crate::{tag::Tag, todo::TodoStatus};

pub struct Node {
    id: String,
    version: u32,

    name: String,
    content: String,

    is_private: bool,
    todo_status: TodoStatus,
    tags: Vec<Tag>,

    parent_id: String,
    prev_sliding_id: Option<String>,

    create_date: usize,
    first_version_date: usize,
}

pub trait NodeMapper {
    fn update_or_insert_node(&self, node: &Node) -> anyhow::Result<()>;
    fn delete_node_by_id(&self, id: &str) -> anyhow::Result<()>;
    fn query_sorted_nodes(&self, ancestor: &str) -> Vec<Node>;

    fn move_nodes(&self, node_id: &str, parent_id: &str, prev_slibing: Option<&str>);
}
