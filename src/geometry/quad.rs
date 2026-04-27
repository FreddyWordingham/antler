use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{
    geometry::{Aabb, Bounded, Traceable},
    tracing::{ObjectHit, ObjectRay},
};

const PARALLEL_EPSILON: f32 = 1.0e-6;
const BOUNDS_THICKNESS: f32 = 1.0e-4;

pub struct Quad {
    pub position: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,
    pub size: [f32; 2],
}

impl Quad {
    pub fn new(position: Point3<f32>, normal: Unit<Vector3<f32>>, size: [f32; 2]) -> Self {
        Self { position, normal, size }
    }

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

impl Bounded for Quad {
    fn bounds(&self) -> Aabb {
        let (tangent, bitangent) = self.tangent_frame();

        let half_width = self.size[0] * 0.5;
        let half_height = self.size[1] * 0.5;

        let corners = [
            self.position + *tangent * half_width + *bitangent * half_height,
            self.position + *tangent * half_width - *bitangent * half_height,
            self.position - *tangent * half_width + *bitangent * half_height,
            self.position - *tangent * half_width - *bitangent * half_height,
        ];

        let mut min = corners[0];
        let mut max = corners[0];

        for corner in corners.into_iter().skip(1) {
            min = Point3::new(min.x.min(corner.x), min.y.min(corner.y), min.z.min(corner.z));
            max = Point3::new(max.x.max(corner.x), max.y.max(corner.y), max.z.max(corner.z));
        }

        let pad = Vector3::new(BOUNDS_THICKNESS, BOUNDS_THICKNESS, BOUNDS_THICKNESS);

        Aabb {
            min: min - pad,
            max: max + pad,
        }
    }
}

impl Traceable for Quad {
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

        let (tangent, bitangent) = self.tangent_frame();

        let local_x = offset.dot(&tangent);
        let local_y = offset.dot(&bitangent);

        let half_width = self.size[0] * 0.5;
        let half_height = self.size[1] * 0.5;

        if local_x.abs() > half_width || local_y.abs() > half_height {
            return None;
        }

        let shading_normal = if denom < 0.0 { self.normal } else { -self.normal };

        let u = (local_x / self.size[0]) + 0.5;
        let v = (local_y / self.size[1]) + 0.5;

        Some(ObjectHit {
            distance,
            position,
            normal: shading_normal,
            uv: Point2::new(u, v),
        })
    }

    #[inline]
    fn trace_distance(&self, ray: &ObjectRay) -> Option<f32> {
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

        let (tangent, bitangent) = self.tangent_frame();

        let local_x = offset.dot(&tangent);
        let local_y = offset.dot(&bitangent);

        let half_width = self.size[0] * 0.5;
        let half_height = self.size[1] * 0.5;

        (local_x.abs() <= half_width && local_y.abs() <= half_height).then_some(distance)
    }
}
