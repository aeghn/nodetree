use std::str::FromStr;

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    model::{node::NodeId, todo::TodoEvent},
    parser::toent::todoevent::TodoCreateType,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoCreateReq {
    pub id: NodeId,
    pub todo_event: Option<TodoEvent>,
    pub create_type: TodoCreateType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeTodoHistory {
    pub id: NodeId,
    pub todo_event: (Option<TodoEvent>, chrono::DateTime<Utc>),
}

pub fn to_todo_status(input: Option<&str>) -> Option<TodoEvent> {
    match input {
        Some(e) => match TodoEvent::from_str(e) {
            Ok(o) => Some(o),
            Err(_) => None,
        },
        None => None,
    }
}

#[async_trait]
pub trait TodoMapper {
    async fn insert_todo_and_update(&self, req: &TodoCreateReq) -> anyhow::Result<()>;
}
