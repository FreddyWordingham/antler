use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{aabb::Aabb, bounded::Bounded, contact::Contact, ray::Ray, traceable::Traceable};

const TORUS_EPSILON: f32 = 1.0e-5;
const TORUS_SCAN_STEPS: usize = 128;
const TORUS_BISECTION_STEPS: usize = 32;

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
    fn implicit(&self, position: Vector3<f32>) -> f32 {
        let r_major = self.major_radius;
        let r_minor = self.minor_radius;

        let x2 = position.x * position.x;
        let y2 = position.y * position.y;
        let z2 = position.z * position.z;

        let s = r_minor.mul_add(-r_minor, r_major.mul_add(r_major, x2 + y2 + z2));

        s.mul_add(s, -(4.0 * r_major * r_major * (x2 + z2)))
    }

    #[inline]
    fn normal_at_local(&self, position: Vector3<f32>) -> Unit<Vector3<f32>> {
        let r_major = self.major_radius;

        let x2 = position.x * position.x;
        let y2 = position.y * position.y;
        let z2 = position.z * position.z;

        let s = self
            .minor_radius
            .mul_add(-self.minor_radius, r_major.mul_add(r_major, x2 + y2 + z2));

        Unit::new_normalize(Vector3::new(
            4.0 * position.x * (2.0 * r_major).mul_add(-r_major, s),
            4.0 * position.y * s,
            4.0 * position.z * (2.0 * r_major).mul_add(-r_major, s),
        ))
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

        if t1 <= TORUS_EPSILON {
            return None;
        }

        let start = t0.max(TORUS_EPSILON);
        let end = t1.min(max_distance);

        (start < end).then_some((start, end))
    }

    fn distance_unchecked(&self, ray: &Ray, max_distance: f32) -> Option<f32> {
        let (start, end) = self.bounding_sphere_interval(ray, max_distance)?;

        let mut previous_t = start;
        let mut previous_value = self.implicit(self.local_at(ray, previous_t));

        if previous_value.abs() <= TORUS_EPSILON {
            return Some(previous_t);
        }

        for step in 1..=TORUS_SCAN_STEPS {
            let t = (end - start).mul_add(step as f32 / TORUS_SCAN_STEPS as f32, start);
            let value = self.implicit(self.local_at(ray, t));

            if value.abs() <= TORUS_EPSILON {
                return Some(t);
            }

            if previous_value.signum() != value.signum() {
                return Some(self.refine_root(ray, previous_t, t));
            }

            previous_t = t;
            previous_value = value;
        }

        None
    }

    fn refine_root(&self, ray: &Ray, mut lo: f32, mut hi: f32) -> f32 {
        let mut lo_value = self.implicit(self.local_at(ray, lo));

        for _ in 0..TORUS_BISECTION_STEPS {
            let mid = (lo + hi) * 0.5;
            let mid_value = self.implicit(self.local_at(ray, mid));

            if lo_value.signum() == mid_value.signum() {
                lo = mid;
                lo_value = mid_value;
            } else {
                hi = mid;
            }
        }

        (lo + hi) * 0.5
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
            .filter(|distance| *distance < max_distance)
    }

    #[inline]
    fn intersection(&self, ray: &Ray, max_distance: f32) -> Option<Contact> {
        let distance = self.distance(ray, max_distance)?;

        let position = ray.origin + *ray.direction * distance;
        let local = position - self.centre;
        let mut normal = self.normal_at_local(local);

        if normal.dot(&ray.direction) > 0.0 {
            normal = -normal;
        }

        Some(Contact::new(distance, position, normal, Point2::new(0.0, 0.0)))
    }
}
