use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    name: String,
    create_time: String,
}

pub trait TagMapper {}
