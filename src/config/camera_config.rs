use serde::{Deserialize, Serialize};

use crate::{
    camera::{CameraEnum, Orthographic, Perspective},
    config::{Vec3, defaults},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CameraConfig {
    Perspective {
        position: Vec3,
        look_at: Vec3,
        #[serde(default = "defaults::z_axis")]
        up: Vec3,
        vertical_fov_radians: f32,
    },
    Orthographic {
        position: Vec3,
        look_at: Vec3,
        #[serde(default = "defaults::z_axis")]
        up: Vec3,
        size: [f32; 2],
    },
}

impl CameraConfig {
    pub fn build(self) -> CameraEnum {
        match self {
            CameraConfig::Perspective {
                position,
                look_at,
                up,
                vertical_fov_radians,
            } => Perspective::new(position.into(), look_at.into(), up.into(), vertical_fov_radians).into(),

            CameraConfig::Orthographic {
                position,
                look_at,
                up,
                size,
            } => Orthographic::new(position.into(), look_at.into(), up.into(), size[0], size[1]).into(),
        }
    }
}
