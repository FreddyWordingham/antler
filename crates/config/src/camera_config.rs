use antler_camera::{Camera, Orthographic, Perspective};
use serde::{Deserialize, Serialize};

use crate::vec3::Vec3;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CameraConfig {
    Orthographic {
        position: Vec3,
        #[serde(default = "default_look_at")]
        look_at: Vec3,
        #[serde(default = "default_up")]
        up: Vec3,
        #[serde(default = "default_size")]
        size: [f32; 2],
    },
    Perspective {
        position: Vec3,
        #[serde(default = "default_look_at")]
        look_at: Vec3,
        #[serde(default = "default_up")]
        up: Vec3,
        #[serde(default = "default_vertical_fov")]
        vertical_fov: f32,
    },
}

impl CameraConfig {
    pub fn build(self) -> Camera {
        match self {
            Self::Orthographic {
                position,
                look_at,
                up,
                size,
            } => Orthographic::new(position.into(), look_at.into(), up.into(), size).into(),
            Self::Perspective {
                position,
                look_at,
                up,
                vertical_fov,
            } => Perspective::new(position.into(), look_at.into(), up.into(), vertical_fov.to_radians()).into(),
        }
    }
}

const fn default_look_at() -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

const fn default_up() -> Vec3 {
    Vec3::new(0.0, 0.0, 1.0)
}

const fn default_size() -> [f32; 2] {
    [10.0, 10.0]
}

const fn default_vertical_fov() -> f32 {
    45.0
}
