use serde::{Deserialize, Serialize};

pub struct TodoObject {
    start_status: String,
    end_status: String,
    start_time: usize,
    end_time: usize,
}

pub trait TodoMapper {}
