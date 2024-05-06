use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{AsRefStr, EnumString};

pub mod v1;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BackupVersion {
    V1,
}

#[derive(Serialize, Deserialize, Clone, Debug, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum BackupContent {
    Nodes,
    NodesHistory,
    Asset,
    AssetFiles,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Backup {
    version: BackupVersion,
    content_type: BackupContent,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    value: Value,
}
