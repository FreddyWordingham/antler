use antler_colour::Rgb;
use antler_light::{Directional, Light};
use serde::{Deserialize, Serialize};

use crate::vec3::Vec3;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum LightConfig {
    Directional {
        direction: Vec3,
        radiance: Rgb,
        #[serde(default)]
        angular_radius: Option<f32>,
        #[serde(default)]
        samples: Option<usize>,
    },
}

impl LightConfig {
    pub fn build(self) -> Light {
        match self {
            Self::Directional {
                direction,
                radiance,
                angular_radius,
                samples,
            } => Directional::new(
                direction.into(),
                radiance,
                angular_radius.map(|r| r.to_radians()),
                samples,
            ),
        }
        .into()
    }
}
