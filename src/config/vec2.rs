use nalgebra::{Point2, Unit, Vector2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Vec2(pub [f32; 2]);

impl Into<Point2<f32>> for Vec2 {
    fn into(self) -> Point2<f32> {
        Point2::new(self.0[0], self.0[1])
    }
}

impl Into<Vector2<f32>> for Vec2 {
    fn into(self) -> Vector2<f32> {
        Vector2::new(self.0[0], self.0[1])
    }
}

impl Into<Unit<Vector2<f32>>> for Vec2 {
    fn into(self) -> Unit<Vector2<f32>> {
        Unit::new_normalize(self.into())
    }
}
