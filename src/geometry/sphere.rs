use nalgebra::{Point3, Vector3};

use crate::geometry::Bounded;

pub struct Sphere {
    pub centre: Point3<f32>,
    pub radius: f32,
}

impl Bounded for Sphere {
    fn bounds(&self) -> crate::geometry::Aabb {
        crate::geometry::Aabb {
            min: self.centre - Vector3::new(self.radius, self.radius, self.radius),
            max: self.centre + Vector3::new(self.radius, self.radius, self.radius),
        }
    }
}
