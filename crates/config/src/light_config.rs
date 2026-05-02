use antler_colour::Rgb;
use antler_light::{Directional, Light, Point};
use serde::{Deserialize, Serialize};

use crate::vec3::Vec3;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum LightConfig {
    Directional {
        direction: Vec3,
        colour: Rgb,
        #[serde(default)]
        angular_radius: Option<f32>,
        #[serde(default)]
        samples: Option<usize>,
    },
    Point {
        position: Vec3,
        colour: Rgb,
        intensity: f32,
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
                colour: radiance,
                angular_radius,
                samples,
            } => Directional::new(direction.into(), radiance, angular_radius.map(f32::to_radians), samples).into(),
            Self::Point {
                position,
                colour,
                intensity,
                angular_radius,
                samples,
            } => Point::new(
                position.into(),
                colour,
                intensity,
                angular_radius.map(f32::to_radians),
                samples,
            )
            .into(),
        }
    }
}
