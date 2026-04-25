use serde::{Deserialize, Serialize};

use crate::material::{MaterialEnum, Mirror, Opaque, Reflective};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialConfig {
    Mirror,
    Opaque,
    Reflective { reflectivity: f32 },
}

impl MaterialConfig {
    pub fn build(self) -> MaterialEnum {
        match self {
            MaterialConfig::Mirror => Mirror::new().into(),
            MaterialConfig::Opaque => Opaque::new().into(),
            MaterialConfig::Reflective { reflectivity } => Reflective::new(reflectivity).into(),
        }
    }
}
