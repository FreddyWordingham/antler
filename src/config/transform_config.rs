use nalgebra::{Similarity3, Translation3, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};

use crate::config::{Vec3, defaults};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformConfig {
    #[serde(default)]
    pub position: Vec3,
    #[serde(default)]
    pub rotation: Vec3,
    #[serde(default = "defaults::one_f32")]
    pub scale: f32,
}

impl Default for TransformConfig {
    fn default() -> Self {
        Self {
            position: Vec3::default(),
            rotation: Vec3::default(),
            scale: defaults::one_f32(),
        }
    }
}

impl From<TransformConfig> for Similarity3<f32> {
    fn from(value: TransformConfig) -> Self {
        let translation = Translation3::from(Vector3::from(value.position));
        let rotation = UnitQuaternion::from_euler_angles(value.rotation.0[0], value.rotation.0[1], value.rotation.0[2]);

        Similarity3::from_parts(translation, rotation, value.scale)
    }
}
