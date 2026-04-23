use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{
    geometry::{Aabb, Bounded, Traceable},
    tracing::{ObjectHit, ObjectRay},
};

const DET_EPSILON: f32 = 1.0e-8;
const BOUNDS_THICKNESS: f32 = 1.0e-4;

pub struct Triangle {
    pub vertices: [Point3<f32>; 3],
}

impl Triangle {
    #[inline]
    pub fn edge_ab(&self) -> Vector3<f32> {
        self.vertices[1] - self.vertices[0]
    }

    #[inline]
    pub fn edge_ac(&self) -> Vector3<f32> {
        self.vertices[2] - self.vertices[0]
    }

    #[inline]
    pub fn geometric_normal(&self) -> Unit<Vector3<f32>> {
        Unit::new_normalize(self.edge_ab().cross(&self.edge_ac()))
    }
}

impl Bounded for Triangle {
    fn bounds(&self) -> Aabb {
        let min = Point3::new(
            self.vertices[0].x.min(self.vertices[1].x).min(self.vertices[2].x),
            self.vertices[0].y.min(self.vertices[1].y).min(self.vertices[2].y),
            self.vertices[0].z.min(self.vertices[1].z).min(self.vertices[2].z),
        );

        let max = Point3::new(
            self.vertices[0].x.max(self.vertices[1].x).max(self.vertices[2].x),
            self.vertices[0].y.max(self.vertices[1].y).max(self.vertices[2].y),
            self.vertices[0].z.max(self.vertices[1].z).max(self.vertices[2].z),
        );

        let pad = Vector3::new(BOUNDS_THICKNESS, BOUNDS_THICKNESS, BOUNDS_THICKNESS);

        Aabb {
            min: min - pad,
            max: max + pad,
        }
    }
}

impl Traceable for Triangle {
    fn trace(&self, ray: &ObjectRay) -> Option<ObjectHit> {
        let edge_ab = self.edge_ab();
        let edge_ac = self.edge_ac();

        let p_vec = ray.direction.cross(&edge_ac);
        let det = edge_ab.dot(&p_vec);

        if det.abs() < DET_EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let t_vec = ray.origin - self.vertices[0];

        let u = t_vec.dot(&p_vec) * inv_det;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q_vec = t_vec.cross(&edge_ab);
        let v = ray.direction.dot(&q_vec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let distance = edge_ac.dot(&q_vec) * inv_det;
        if distance <= 0.0 {
            return None;
        }

        let position = ray.origin + *ray.direction * distance;

        let outward_normal = self.geometric_normal();
        let normal = if outward_normal.dot(&ray.direction) < 0.0 {
            outward_normal
        } else {
            -outward_normal
        };

        Some(ObjectHit {
            distance,
            position,
            normal,
            uv: Point2::new(u, v),
        })
    }
}
