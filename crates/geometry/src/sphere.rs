use std::f32::consts::{PI, TAU};

use nalgebra::{Point2, Point3, Unit, Vector3};
use rand::{Rng, RngExt};

use crate::{
    aabb::Aabb, bounded::Bounded, config::MIN_RAY_DISTANCE, contact::Contact, ray::Ray, sample::Sample,
    sampleable::Sampleable, traceable::Traceable,
};

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
    fn hit(&self, ray: &Ray, max_distance: f32) -> bool {
        self.distance(ray, max_distance).is_some()
    }

    #[inline]
    fn distance(&self, ray: &Ray, max_distance: f32) -> Option<f32> {
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

        (distance > MIN_RAY_DISTANCE && distance < max_distance).then_some(distance)
    }

    #[inline]
    fn intersection(&self, ray: &Ray, max_distance: f32) -> Option<Contact> {
        let distance = self.distance(ray, max_distance)?;

        let position = ray.origin + *ray.direction * distance;
        let normal = Unit::new_normalize(position - self.centre);

        let local = position - self.centre;
        let theta = (-local.y / self.radius).acos();
        let phi = local.z.atan2(local.x);
        let u = (phi + PI) / TAU;
        let v = theta / PI;

        Some(Contact::new(distance, position, normal, Point2::new(u, v), None))
    }
}

impl Sampleable for Sphere {
    #[inline]
    fn area(&self) -> f32 {
        4.0 * std::f32::consts::PI * self.radius * self.radius
    }

    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> Sample {
        let u1 = rng.random::<f32>();
        let u2 = rng.random::<f32>();

        let z = 1.0 - 2.0 * u1;
        let r = (1.0 - z * z).sqrt();
        let phi = TAU * u2;

        let x = r * phi.cos();
        let y = r * phi.sin();

        let normal = Unit::new_unchecked(Vector3::new(x, y, z));
        let position = self.centre + normal.into_inner() * self.radius;

        Sample {
            position,
            normal,
            pdf_area: 1.0 / self.area(),
        }
    }
}
