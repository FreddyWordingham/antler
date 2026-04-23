use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{
    geometry::{Aabb, Bounded, Traceable},
    tracing::{ObjectHit, ObjectRay},
};

const PARALLEL_EPSILON: f32 = 1.0e-6;
const BOUNDS_THICKNESS: f32 = 1.0e-4;

pub struct Circle {
    pub position: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,
    pub radius: f32,
}

impl Circle {
    #[inline]
    fn tangent_frame(&self) -> (Unit<Vector3<f32>>, Unit<Vector3<f32>>) {
        let helper = if self.normal.x.abs() < 0.9 {
            Vector3::x_axis()
        } else {
            Vector3::y_axis()
        };

        let tangent = Unit::new_normalize(self.normal.cross(&helper));
        let bitangent = Unit::new_normalize(self.normal.cross(&tangent));
        (tangent, bitangent)
    }
}

impl Bounded for Circle {
    fn bounds(&self) -> Aabb {
        let n = self.normal.into_inner().map(|x| x.abs());
        let radial = Vector3::new(
            self.radius * (1.0 - n.x * n.x).sqrt(),
            self.radius * (1.0 - n.y * n.y).sqrt(),
            self.radius * (1.0 - n.z * n.z).sqrt(),
        );

        let thickness = Vector3::new(
            BOUNDS_THICKNESS.max(BOUNDS_THICKNESS * n.x),
            BOUNDS_THICKNESS.max(BOUNDS_THICKNESS * n.y),
            BOUNDS_THICKNESS.max(BOUNDS_THICKNESS * n.z),
        );

        let extent = radial + thickness;

        Aabb {
            min: self.position - extent,
            max: self.position + extent,
        }
    }
}

impl Traceable for Circle {
    fn trace(&self, ray: &ObjectRay) -> Option<ObjectHit> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < PARALLEL_EPSILON {
            return None;
        }

        let distance = (self.position - ray.origin).dot(&self.normal) / denom;
        if distance <= 0.0 {
            return None;
        }

        let position = ray.origin + *ray.direction * distance;
        let offset = position - self.position;

        let radial_sq = offset.norm_squared() - offset.dot(&self.normal).powi(2);
        if radial_sq > self.radius * self.radius {
            return None;
        }

        let shading_normal = if denom < 0.0 { self.normal } else { -self.normal };

        let (tangent, bitangent) = self.tangent_frame();
        let u = 0.5 + 0.5 * offset.dot(&tangent) / self.radius;
        let v = 0.5 + 0.5 * offset.dot(&bitangent) / self.radius;

        Some(ObjectHit {
            distance,
            position,
            normal: shading_normal,
            uv: Point2::new(u, v),
        })
    }
}
