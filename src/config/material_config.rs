use serde::{Deserialize, Serialize};

use crate::material::{MaterialEnum, Opaque};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MaterialConfig {
    Opaque,
}

impl MaterialConfig {
    pub fn build(self) -> MaterialEnum {
        match self {
            MaterialConfig::Opaque => Opaque.into(),
        }
    }
}
