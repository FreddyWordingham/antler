use nalgebra::{Point2, Unit, Vector2};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec2(pub [f32; 2]);

impl From<Vec2> for Point2<f32> {
    fn from(value: Vec2) -> Self {
        Point2::new(value.0[0], value.0[1])
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

impl Serialize for Vec2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Vec2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec2(<[f32; 2]>::deserialize(deserializer)?))
    }
}
