use std::f32::consts::{PI, TAU};

use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{aabb::Aabb, bounded::Bounded, intersection::Intersection, ray::Ray, traceable::Traceable};

pub struct Sphere {
    centre: Point3<f32>,
    radius: f32,
}

impl Sphere {
    #[must_use]
    pub const fn new(centre: Point3<f32>, radius: f32) -> Self {
        Self { centre, radius }
    }
}

impl Bounded for Sphere {
    fn bounds(&self) -> Aabb {
        let min = self.centre - Vector3::new(self.radius, self.radius, self.radius);
        let max = self.centre + Vector3::new(self.radius, self.radius, self.radius);
        Aabb::new(min, max)
    }
}

impl Traceable for Sphere {
    #[inline]
    fn distance(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.centre;

        let half_b = oc.dot(&ray.direction);
        let c = self.radius.mul_add(-self.radius, oc.dot(&oc));

        let discriminant = half_b.mul_add(half_b, -c);
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut distance = -half_b - sqrt_d;
        if distance <= 0.0 {
            distance = -half_b + sqrt_d;
        }

        (distance > 0.0).then_some(distance)
    }

    #[inline]
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let distance = self.distance(ray)?;

        let position = ray.origin + *ray.direction * distance;
        let normal = Unit::new_normalize(position - self.centre);

        let local = position - self.centre;
        let theta = (-local.y / self.radius).acos();
        let phi = local.z.atan2(local.x);
        let u = (phi + PI) / TAU;
        let v = theta / PI;

        Some(Intersection::new(distance, position, normal, Point2::new(u, v)))
    }
}
