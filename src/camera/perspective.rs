use nalgebra::{Isometry3, Point2, Point3, Unit, Vector3};

use crate::{
    camera::Camera,
    geometry::Ray,
    tracing::{Probe, WorldRay},
};

pub struct Perspective {
    pub transform: Isometry3<f32>,
    pub vertical_fov_radians: f32,
}

impl Perspective {
    pub fn new(position: Point3<f32>, look_at: Point3<f32>, up: Unit<Vector3<f32>>, vertical_fov_radians: f32) -> Self {
        Self {
            transform: Isometry3::look_at_rh(&position, &look_at, &up).inverse(),
            vertical_fov_radians,
        }
    }
}

impl Camera for Perspective {
    fn emit(&self, uv: Point2<f32>, resolution: [usize; 2]) -> Probe {
        let aspect_ratio = resolution[0] as f32 / resolution[1] as f32;
        let tan_half_fov = (self.vertical_fov_radians * 0.5).tan();

        let x = (2.0 * uv.x - 1.0) * aspect_ratio * tan_half_fov;
        let y = (1.0 - 2.0 * uv.y) * tan_half_fov;

        let local_origin = Point3::new(0.0, 0.0, 0.0);
        let local_direction = Unit::new_normalize(Vector3::new(x, y, -1.0));

        let world_origin = self.transform.transform_point(&local_origin);
        let world_direction = Unit::new_normalize(self.transform.transform_vector(&local_direction.into_inner()));

        Probe::new(WorldRay::new(Ray {
            origin: world_origin,
            direction: world_direction,
        }))
    }
}
