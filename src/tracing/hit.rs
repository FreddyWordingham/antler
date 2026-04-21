use nalgebra::{Point2, Point3, Unit, Vector3};

pub struct Hit {
    pub distance: f32,
    pub position: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,
    pub uv: Point2<f32>,
}

impl Hit {
    pub fn is_interior(&self, direction: &Unit<Vector3<f32>>) -> bool {
        self.normal.dot(direction) > 0.0
    }
}
