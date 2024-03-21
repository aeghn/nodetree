use serde::{Deserialize, Serialize};

use crate::constants;

pub mod alarm;
pub mod node;
pub mod tag;
pub mod todo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicConstants {
    pub root: String,
}

impl Default for MagicConstants {
    fn default() -> Self {
        Self {
            root: constants::MAGIC_ROOT_NODE_ID.to_string(),
        }
    }
}
