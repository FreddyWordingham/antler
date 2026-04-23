use serde::{Deserialize, Serialize};

use crate::material::{MaterialEnum, Opaque};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MaterialConfig {
    Opaque,
}

impl From<MaterialConfig> for MaterialEnum {
    fn from(config: MaterialConfig) -> Self {
        match config {
            MaterialConfig::Opaque => Opaque::new().into(),
        }
    }
}
