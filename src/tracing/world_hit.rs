use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::id::ObjectId;

pub struct WorldHit {
    pub object_id: ObjectId,
    pub distance: f32,
    pub position: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,
    pub uv: Point2<f32>,
}

impl WorldHit {
    #[inline]
    pub fn is_interior(&self, direction: &Unit<Vector3<f32>>) -> bool {
        self.normal.dot(direction) > 0.0
    }
}
