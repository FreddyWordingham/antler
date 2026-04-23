use nalgebra::{Point2, Unit, Vector2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Vec2(pub [f32; 2]);

impl From<Point2<f32>> for Vec2 {
    fn from(value: Point2<f32>) -> Self {
        Vec2([value.x, value.y])
    }
}

impl From<Vec2> for Vector2<f32> {
    fn from(value: Vec2) -> Self {
        Vector2::new(value.0[0], value.0[1])
    }
}

impl From<Vec2> for Unit<Vector2<f32>> {
    fn from(value: Vec2) -> Self {
        Unit::new_normalize(value.into())
    }
}
