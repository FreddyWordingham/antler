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

impl From<LightConfig> for LightEnum {
    fn from(config: LightConfig) -> Self {
        match config {
            LightConfig::Directional { direction, radiance } => {
                DirectionalLight::new(direction.into(), radiance).into()
            }
        }
    }
}
