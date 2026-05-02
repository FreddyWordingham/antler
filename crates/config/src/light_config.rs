use antler_colour::Rgb;
use antler_light::{Directional, Environment, Light, Point};
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
    Environment {
        zenith: Rgb,
        horizon: Rgb,
        #[serde(default = "default_up")]
        up: Vec3,
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
            Self::Environment {
                zenith,
                horizon,
                up,
                samples,
            } => Environment::new(horizon, zenith, up.into(), samples).into(),
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

fn default_up() -> Vec3 {
    Vec3::new(0.0, 0.0, 1.0)
}
