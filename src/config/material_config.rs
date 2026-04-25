use serde::{Deserialize, Serialize};

use crate::material::{MaterialEnum, Mirror, Opaque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialConfig {
    Opaque,
    Mirror,
}

impl MaterialConfig {
    pub fn build(self) -> MaterialEnum {
        match self {
            MaterialConfig::Opaque => Opaque::new().into(),
            MaterialConfig::Mirror => Mirror::new().into(),
        }
    }
}
