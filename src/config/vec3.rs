use nalgebra::{Point3, Unit, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Vec3(pub [f32; 3]);

impl Into<Point3<f32>> for Vec3 {
    fn into(self) -> Point3<f32> {
        Point3::new(self.0[0], self.0[1], self.0[2])
    }
}

impl Into<Vector3<f32>> for Vec3 {
    fn into(self) -> Vector3<f32> {
        Vector3::new(self.0[0], self.0[1], self.0[2])
    }
}

impl Into<Unit<Vector3<f32>>> for Vec3 {
    fn into(self) -> Unit<Vector3<f32>> {
        Unit::new_normalize(self.into())
    }
}
