use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{AsRefStr, EnumString};

use self::v1::table::TableRow;

pub mod v1;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BackupVersion {
    V1,
}
