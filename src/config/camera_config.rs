use serde::{Deserialize, Serialize};

use crate::config::{Vec3, defaults};

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
