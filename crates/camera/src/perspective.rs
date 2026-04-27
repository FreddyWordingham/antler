use antler_geometry::Ray;
use nalgebra::{Isometry3, Point2, Point3, Unit, Vector3};

use crate::camera::Camera;

pub struct Perspective {
    pub transform: Isometry3<f32>,
    pub vertical_fov: f32,
}

impl Perspective {
    #[must_use]
    pub fn new(position: Point3<f32>, look_at: Point3<f32>, up: Unit<Vector3<f32>>, vertical_fov: f32) -> Self {
        Self {
            transform: Isometry3::look_at_rh(&position, &look_at, &up).inverse(),
            vertical_fov,
        }
    }
}

impl Camera for Perspective {
    fn emit(&self, resolution: [usize; 2], uv: Point2<f32>) -> Ray {
        let aspect_ratio = resolution[0] as f32 / resolution[1] as f32;
        let tan_half_fov = (self.vertical_fov * 0.5).tan();

        let x = 2.0f32.mul_add(uv.x, -1.0) * aspect_ratio * tan_half_fov;
        let y = 2.0f32.mul_add(-uv.y, 1.0) * tan_half_fov;

        let local_origin = Point3::new(0.0, 0.0, 0.0);
        let local_direction = Unit::new_normalize(Vector3::new(x, y, -1.0));

        let origin = self.transform.transform_point(&local_origin);
        let direction = Unit::new_normalize(self.transform.transform_vector(&local_direction.into_inner()));

        Ray::new(origin, direction)
    }
}
