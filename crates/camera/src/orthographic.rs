use antler_geometry::Ray;
use nalgebra::{Isometry3, Point2, Point3, Unit, Vector3};

use crate::camera::Camera;

pub struct Orthographic {
    pub transform: Isometry3<f32>,
    pub size: [f32; 2],
}

impl Orthographic {
    pub fn new(position: Point3<f32>, look_at: Point3<f32>, up: Unit<Vector3<f32>>, size: [f32; 2]) -> Self {
        Self {
            transform: Isometry3::look_at_rh(&position, &look_at, &up).inverse(),
            size,
        }
    }
}

impl Camera for Orthographic {
    fn emit(&self, _resolution: [usize; 2], uv: Point2<f32>) -> Ray {
        let x = (2.0 * uv.x - 1.0) * self.size[0] * 0.5;
        let y = (1.0 - 2.0 * uv.y) * self.size[1] * 0.5;

        let local_origin = Point3::new(x, y, 0.0);
        let local_direction = Unit::new_normalize(Vector3::new(0.0, 0.0, -1.0));

        let origin = self.transform.transform_point(&local_origin);
        let direction = Unit::new_normalize(self.transform.transform_vector(&local_direction.into_inner()));

        Ray { origin, direction }
    }
}
