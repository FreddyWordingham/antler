use antler_camera::{Camera, Orthographic, Perspective};
use serde::{Deserialize, Serialize};

use crate::vec3::Vec3;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CameraConfig {
    Orthographic {
        position: Vec3,
        look_at: Vec3,
        up: Vec3,
        size: [f32; 2],
    },
    Perspective {
        position: Vec3,
        look_at: Vec3,
        up: Vec3,
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
