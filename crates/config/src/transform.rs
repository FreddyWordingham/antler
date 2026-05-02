use nalgebra::{Similarity3, Translation3, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};

use crate::vec3::Vec3;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transform {
    #[serde(default)]
    pub translation: Vec3,
    #[serde(default)]
    pub rotation: Vec3,
    #[serde(default = "one_f32")]
    pub scale: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::default(),
            rotation: Vec3::default(),
            scale: 1.0,
        }
    }
}

impl From<Transform> for Similarity3<f32> {
    fn from(value: Transform) -> Self {
        let translation = Translation3::from(Vector3::from(value.translation));
        let rotation = UnitQuaternion::from_euler_angles(
            value.rotation.0[0].to_radians(),
            value.rotation.0[1].to_radians(),
            value.rotation.0[2].to_radians(),
        );

        Self::from_parts(translation, rotation, value.scale)
    }
}

const fn one_f32() -> f32 {
    1.0
}
