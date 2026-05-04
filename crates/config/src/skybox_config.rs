use antler_colour::Rgb;
use antler_skybox::{Constant, Gradient, Skybox};
use serde::{Deserialize, Serialize};

use crate::{gradient_config::GradientConfig, vec3::Vec3};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SkyboxConfig {
    Constant {
        colour: Rgb,
    },
    Gradient {
        gradient: GradientConfig,
        #[serde(default = "default_power")]
        power: f32,
        #[serde(default = "default_up")]
        up: Vec3,
    },
}

impl SkyboxConfig {
    pub fn build(self) -> Skybox {
        match self {
            Self::Constant { colour } => Constant::new(colour).into(),
            Self::Gradient { gradient, power, up } => Gradient::new(gradient.into(), power, up.into()).into(),
        }
    }
}

const fn default_power() -> f32 {
    1.0
}

const fn default_up() -> Vec3 {
    Vec3::new(0.0, 0.0, 1.0)
}
