use std::f32::consts::{PI, TAU};

use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{aabb::Aabb, bounded::Bounded, config::MIN_RAY_DISTANCE, contact::Contact, ray::Ray, traceable::Traceable};

pub struct Capsule {
    a: Point3<f32>,
    b: Point3<f32>,
    radius: f32,
}

impl Capsule {
    #[must_use]
    pub fn new(a: Point3<f32>, b: Point3<f32>, radius: f32) -> Self {
        assert!((b - a).norm_squared() > 1.0e-12, "Capsule endpoints must differ");
        assert!(radius > 0.0, "Capsule radius must be positive");

        Self { a, b, radius }
    }

    fn distance_unchecked(&self, ray: &Ray) -> Option<f32> {
        let ba = self.b - self.a;
        let oa = ray.origin - self.a;
        let rd = ray.direction.into_inner();

        let baba = ba.dot(&ba);
        let bard = ba.dot(&rd);
        let baoa = ba.dot(&oa);
        let rdoa = rd.dot(&oa);
        let oaoa = oa.dot(&oa);

        let radius2 = self.radius * self.radius;

        let a = bard.mul_add(-bard, baba);
        let b = baba.mul_add(rdoa, -(baoa * bard));
        let c = radius2.mul_add(-baba, baba.mul_add(oaoa, -(baoa * baoa)));

        let h = b.mul_add(b, -(a * c));
        if h < 0.0 {
            return None;
        }

        let mut t = (-b - h.sqrt()) / a;
        let y = t.mul_add(bard, baoa);

        if y > 0.0 && y < baba {
            return (t > MIN_RAY_DISTANCE).then_some(t);
        }

        let oc = if y <= 0.0 { oa } else { ray.origin - self.b };

        let b = rd.dot(&oc);
        let c = oc.dot(&oc) - radius2;
        let h = b * b - c;

        if h < 0.0 {
            return None;
        }

        t = -b - h.sqrt();

        (t > MIN_RAY_DISTANCE).then_some(t)
    }

    fn normal(&self, position: Point3<f32>) -> Unit<Vector3<f32>> {
        let ba = self.b - self.a;
        let pa = position - self.a;

        let h = (pa.dot(&ba) / ba.dot(&ba)).clamp(0.0, 1.0);
        let closest = self.a + ba * h;

        Unit::new_normalize(position - closest)
    }

    fn uv(&self, position: Point3<f32>) -> Point2<f32> {
        let axis = self.b - self.a;
        let length = axis.norm();
        let axis_dir = axis / length;

        let pa = position - self.a;
        let h = (pa.dot(&axis_dir) / length).clamp(0.0, 1.0);
        let centre = self.a + axis * h;

        let radial = Unit::new_normalize(position - centre);

        let helper = if axis_dir.x.abs() < 0.9 {
            Vector3::x()
        } else {
            Vector3::y()
        };

        let tangent = Unit::new_normalize(helper.cross(&axis_dir));
        let bitangent = Unit::new_normalize(axis_dir.cross(&tangent));

        let x = radial.dot(&tangent);
        let y = radial.dot(&bitangent);

        let u = (y.atan2(x) + PI) / TAU;
        let v = h;

        Point2::new(u, v)
    }
}

impl Bounded for Capsule {
    #[inline]
    fn bounds(&self) -> Aabb {
        let radius = Vector3::new(self.radius, self.radius, self.radius);

        Aabb::new(
            Point3::new(self.a.x.min(self.b.x), self.a.y.min(self.b.y), self.a.z.min(self.b.z)) - radius,
            Point3::new(self.a.x.max(self.b.x), self.a.y.max(self.b.y), self.a.z.max(self.b.z)) + radius,
        )
    }
}

impl Traceable for Capsule {
    #[inline]
    fn hit(&self, ray: &Ray, max_distance: f32) -> bool {
        self.distance(ray, max_distance).is_some()
    }

    #[inline]
    fn distance(&self, ray: &Ray, max_distance: f32) -> Option<f32> {
        self.distance_unchecked(ray)
            .filter(|distance| *distance > MIN_RAY_DISTANCE && *distance < max_distance)
    }

    #[inline]
    fn intersection(&self, ray: &Ray, max_distance: f32) -> Option<Contact> {
        let distance = self.distance(ray, max_distance)?;

        let position = ray.origin + *ray.direction * distance;
        let mut normal = self.normal(position);

        if normal.dot(&ray.direction) > 0.0 {
            normal = -normal;
        }

        let uv = self.uv(position);
        Some(Contact::new(distance, position, normal, uv, None))
    }
}
