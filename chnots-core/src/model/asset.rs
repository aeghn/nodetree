use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Asset {
    pub id: String,

    pub domain: Option<String>,
    pub ori_file_name: String,

    pub content_type: String,

    pub create_time: DateTime<Utc>,
}
