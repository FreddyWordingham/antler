use std::f32::consts::{PI, TAU};

use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{
    geometry::{Bounded, Traceable},
    tracing::{ObjectHit, ObjectRay},
};

pub struct Sphere {
    pub centre: Point3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(centre: Point3<f32>, radius: f32) -> Self {
        Self { centre, radius }
    }
}

impl Bounded for Sphere {
    fn bounds(&self) -> crate::geometry::Aabb {
        crate::geometry::Aabb {
            min: self.centre - Vector3::new(self.radius, self.radius, self.radius),
            max: self.centre + Vector3::new(self.radius, self.radius, self.radius),
        }
    }
}

impl Traceable for Sphere {
    fn trace(&self, ray: &ObjectRay) -> Option<ObjectHit> {
        let oc = ray.origin - self.centre;

        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut distance = (-half_b - sqrt_d) / a;
        if distance <= 0.0 {
            distance = (-half_b + sqrt_d) / a;

            if distance <= 0.0 {
                return None;
            }
        }

        let position = ray.origin + *ray.direction * distance;
        let outward_normal = Unit::new_normalize(position - self.centre);

        let local = position - self.centre;
        let theta = (-local.y / self.radius).acos();
        let phi = local.z.atan2(local.x);
        let u = (phi + PI) / TAU;
        let v = theta / PI;

        Some(ObjectHit {
            distance,
            position,
            normal: outward_normal,
            uv: Point2::new(u, v),
        })
    }
}
