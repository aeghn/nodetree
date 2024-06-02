use serde::{Deserialize, Serialize};

pub mod v1;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BackupVersion {
    V1,
}
