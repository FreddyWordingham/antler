use nalgebra::{Point3, Unit, Vector3};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3(pub [f32; 3]);

impl From<Vec3> for Point3<f32> {
    fn from(value: Vec3) -> Self {
        Point3::new(value.0[0], value.0[1], value.0[2])
    }
}

impl From<Vec3> for Vector3<f32> {
    fn from(value: Vec3) -> Self {
        Vector3::new(value.0[0], value.0[1], value.0[2])
    }
}

impl From<Vec3> for Unit<Vector3<f32>> {
    fn from(value: Vec3) -> Self {
        Unit::new_normalize(value.into())
    }
}

impl Serialize for Vec3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Vec3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec3(<[f32; 3]>::deserialize(deserializer)?))
    }
}
