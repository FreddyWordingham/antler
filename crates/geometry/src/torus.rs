use std::f32::consts::{PI, TAU};

use nalgebra::{Point2, Point3, Unit, Vector3};
use rand::{Rng, RngExt};

use crate::{
    aabb::Aabb, bounded::Bounded, config::MIN_RAY_DISTANCE, contact::Contact, ray::Ray, sample::Sample,
    sampleable::Sampleable, traceable::Traceable,
};

const TORUS_MAX_STEPS: usize = 256;
const TORUS_HIT_EPSILON: f32 = 1.0e-4;
const TORUS_NORMAL_EPSILON: f32 = 1.0e-3;

pub struct Torus {
    centre: Point3<f32>,
    major_radius: f32,
    minor_radius: f32,
}

impl Torus {
    #[must_use]
    pub const fn new(centre: Point3<f32>, major_radius: f32, minor_radius: f32) -> Self {
        Self {
            centre,
            major_radius,
            minor_radius,
        }
    }

    #[inline]
    fn local_at(&self, ray: &Ray, distance: f32) -> Vector3<f32> {
        ray.origin.coords + ray.direction.into_inner() * distance - self.centre.coords
    }

    #[inline]
    fn signed_distance(&self, p: Vector3<f32>) -> f32 {
        let qx = p.x.hypot(p.z) - self.major_radius;
        let qy = p.y;

        qx.hypot(qy) - self.minor_radius
    }

    fn bounding_sphere_interval(&self, ray: &Ray, max_distance: f32) -> Option<(f32, f32)> {
        let radius = self.major_radius + self.minor_radius;
        let oc = ray.origin - self.centre;

        let half_b = oc.dot(&ray.direction);
        let c = radius.mul_add(-radius, oc.dot(&oc));
        let discriminant = half_b.mul_add(half_b, -c);

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let t0 = -half_b - sqrt_d;
        let t1 = -half_b + sqrt_d;

        if t1 <= MIN_RAY_DISTANCE {
            return None;
        }

        let start = t0.max(MIN_RAY_DISTANCE);
        let end = t1.min(max_distance);

        (start < end).then_some((start, end))
    }

    fn distance_unchecked(&self, ray: &Ray, max_distance: f32) -> Option<f32> {
        let (mut t, end) = self.bounding_sphere_interval(ray, max_distance)?;

        for _ in 0..TORUS_MAX_STEPS {
            let p = self.local_at(ray, t);
            let distance = self.signed_distance(p);

            if distance.abs() <= TORUS_HIT_EPSILON {
                return Some(t);
            }

            t += distance.max(TORUS_HIT_EPSILON * 0.5);

            if t >= end {
                return None;
            }
        }

        None
    }

    #[inline]
    fn normal(&self, p: Vector3<f32>) -> Unit<Vector3<f32>> {
        let e = TORUS_NORMAL_EPSILON;

        let dx = self.signed_distance(p + Vector3::x() * e) - self.signed_distance(p - Vector3::x() * e);
        let dy = self.signed_distance(p + Vector3::y() * e) - self.signed_distance(p - Vector3::y() * e);
        let dz = self.signed_distance(p + Vector3::z() * e) - self.signed_distance(p - Vector3::z() * e);

        Unit::new_normalize(Vector3::new(dx, dy, dz))
    }

    #[inline]
    fn uv(&self, position: Point3<f32>) -> Point2<f32> {
        let local = position - self.centre;

        let major_angle = local.z.atan2(local.x);
        let ring_x = local.x.hypot(local.z) - self.major_radius;
        let minor_angle = local.y.atan2(ring_x);

        Point2::new((major_angle + PI) / TAU, (minor_angle + PI) / TAU)
    }
}

impl Bounded for Torus {
    #[inline]
    fn bounds(&self) -> Aabb {
        let outer = self.major_radius + self.minor_radius;

        Aabb::new(
            self.centre - Vector3::new(outer, self.minor_radius, outer),
            self.centre + Vector3::new(outer, self.minor_radius, outer),
        )
    }
}

impl Traceable for Torus {
    #[inline]
    fn hit(&self, ray: &Ray, max_distance: f32) -> bool {
        self.distance(ray, max_distance).is_some()
    }

    #[inline]
    fn distance(&self, ray: &Ray, max_distance: f32) -> Option<f32> {
        self.distance_unchecked(ray, max_distance)
            .filter(|distance| *distance > MIN_RAY_DISTANCE && *distance < max_distance)
    }

    #[inline]
    fn intersection(&self, ray: &Ray, max_distance: f32) -> Option<Contact> {
        let distance = self.distance(ray, max_distance)?;

        let position = ray.origin + *ray.direction * distance;
        let local = position - self.centre;

        let mut normal = self.normal(local);

        if normal.dot(&ray.direction) > 0.0 {
            normal = -normal;
        }

        Some(Contact::new(distance, position, normal, self.uv(position), None))
    }
}

impl Sampleable for Torus {
    #[inline]
    fn area(&self) -> f32 {
        4.0 * PI * PI * self.major_radius * self.minor_radius
    }

    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> Sample {
        let major_angle = TAU * rng.random::<f32>();

        let minor_angle = loop {
            let candidate = TAU * rng.random::<f32>();
            let accept =
                self.minor_radius.mul_add(candidate.cos(), self.major_radius) / (self.major_radius + self.minor_radius);

            if rng.random::<f32>() <= accept {
                break candidate;
            }
        };

        let major_cos = major_angle.cos();
        let major_sin = major_angle.sin();
        let minor_cos = minor_angle.cos();
        let minor_sin = minor_angle.sin();

        let ring_radius = self.minor_radius.mul_add(minor_cos, self.major_radius);

        let position = Point3::new(
            ring_radius.mul_add(major_cos, self.centre.x),
            self.minor_radius.mul_add(minor_sin, self.centre.y),
            ring_radius.mul_add(major_sin, self.centre.z),
        );

        let normal = Unit::new_normalize(Vector3::new(minor_cos * major_cos, minor_sin, minor_cos * major_sin));

        Sample {
            position,
            normal,
            pdf_area: 1.0 / self.area(),
        }
    }
}
