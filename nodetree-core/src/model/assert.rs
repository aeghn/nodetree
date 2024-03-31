use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Asset {
    pub id: String,

    pub username: Option<String>,
    pub ori_file_name: String,

    pub content_type: String,

    pub create_time: NaiveDateTime,
}
