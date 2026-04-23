use serde::{Deserialize, Serialize};

use crate::{
    colour::Rgb,
    config::Vec3,
    lighting::{DirectionalLight, LightEnum},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LightConfig {
    Directional { direction: Vec3, radiance: Rgb },
}

impl LightConfig {
    pub fn build(self) -> LightEnum {
        match self {
            LightConfig::Directional { direction, radiance } => {
                DirectionalLight::new(direction.into(), radiance).into()
            }
        }
    }
}
