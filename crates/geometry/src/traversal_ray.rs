use nalgebra::{Point3, Vector3};

use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct TraversalRay {
    pub origin: Point3<f32>,
    pub inv_dir: Vector3<f32>,
    pub dir_non_negative: [bool; 3],
}

impl TraversalRay {
    #[must_use]
    #[inline]
    pub fn new(ray: &Ray) -> Self {
        let direction = ray.direction.into_inner();

        Self {
            origin: ray.origin,
            inv_dir: Vector3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z),
            dir_non_negative: [direction.x >= 0.0, direction.y >= 0.0, direction.z >= 0.0],
        }
    }
}
