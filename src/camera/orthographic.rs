use nalgebra::{Isometry3, Point2, Point3, Unit, Vector3};

use crate::{
    camera::Camera,
    geometry::Ray,
    tracing::{Probe, WorldRay},
};

pub struct Orthographic {
    pub transform: Isometry3<f32>,
    pub width: f32,
    pub height: f32,
}

impl Orthographic {
    pub fn new(position: Point3<f32>, look_at: Point3<f32>, up: Unit<Vector3<f32>>, width: f32, height: f32) -> Self {
        Self {
            transform: Isometry3::look_at_rh(&position, &look_at, &up).inverse(),
            width,
            height,
        }
    }
}

impl Camera for Orthographic {
    fn emit(&self, uv: Point2<f32>, _resolution: [usize; 2]) -> Probe {
        let x = (2.0 * uv.x - 1.0) * self.width * 0.5;
        let y = (1.0 - 2.0 * uv.y) * self.height * 0.5;

        let local_origin = Point3::new(x, y, 0.0);
        let local_direction = Unit::new_normalize(Vector3::new(0.0, 0.0, -1.0));

        let world_origin = self.transform.transform_point(&local_origin);
        let world_direction = Unit::new_normalize(self.transform.transform_vector(&local_direction.into_inner()));

        Probe::new(WorldRay::new(Ray {
            origin: world_origin,
            direction: world_direction,
        }))
    }
}
