use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tag {
    name: String,
    create_time: String,
}

pub trait TagMapper {}
