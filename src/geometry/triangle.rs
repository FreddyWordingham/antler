use std::f32::EPSILON;

use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{
    geometry::{Aabb, Bounded, Traceable},
    tracing::{ObjectHit, ObjectRay},
};

const HIT_EPSILON: f32 = 1.0e-6;

pub struct Triangle {
    pub vertices: [Point3<f32>; 3],
    pub normals: Option<[Unit<Vector3<f32>>; 3]>,
    pub uvs: Option<[Point2<f32>; 3]>,
}

impl Triangle {
    #[inline]
    pub fn new(
        vertices: [Point3<f32>; 3],
        normals: Option<[Unit<Vector3<f32>>; 3]>,
        uvs: Option<[Point2<f32>; 3]>,
    ) -> Self {
        Self { vertices, normals, uvs }
    }

    #[inline]
    pub fn face_normal(&self) -> Unit<Vector3<f32>> {
        Unit::new_normalize((self.vertices[1] - self.vertices[0]).cross(&(self.vertices[2] - self.vertices[0])))
    }

    #[inline]
    fn interpolate_position(&self, bary: Vector3<f32>) -> Point3<f32> {
        Point3::from(
            self.vertices[0].coords * bary.x + self.vertices[1].coords * bary.y + self.vertices[2].coords * bary.z,
        )
    }

    #[inline]
    fn interpolate_normal(&self, bary: Vector3<f32>) -> Unit<Vector3<f32>> {
        match &self.normals {
            Some([na, nb, nc]) => {
                let normal = na.into_inner() * bary.x + nb.into_inner() * bary.y + nc.into_inner() * bary.z;

                if normal.norm_squared() > EPSILON {
                    Unit::new_normalize(normal)
                } else {
                    self.face_normal()
                }
            }
            None => self.face_normal(),
        }
    }

    #[inline]
    fn interpolate_uv(&self, bary: Vector3<f32>) -> Point2<f32> {
        match self.uvs {
            Some([uva, uvb, uvc]) => Point2::from(uva.coords * bary.x + uvb.coords * bary.y + uvc.coords * bary.z),
            None => Point2::new(bary.y, bary.z),
        }
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

        Aabb { min, max }
    }
}

impl Traceable for Triangle {
    fn trace(&self, ray: &ObjectRay) -> Option<ObjectHit> {
        let edge_ab = self.vertices[1] - self.vertices[0];
        let edge_ac = self.vertices[2] - self.vertices[0];

        let p_vec = ray.direction.cross(&edge_ac);
        let det = edge_ab.dot(&p_vec);

        if det.abs() < HIT_EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let t_vec = ray.origin - self.vertices[0];

        let beta = t_vec.dot(&p_vec) * inv_det;
        if !(0.0..=1.0).contains(&beta) {
            return None;
        }

        let q_vec = t_vec.cross(&edge_ab);
        let gamma = ray.direction.dot(&q_vec) * inv_det;
        if gamma < 0.0 || beta + gamma > 1.0 {
            return None;
        }

        let distance = edge_ac.dot(&q_vec) * inv_det;
        if distance <= HIT_EPSILON {
            return None;
        }

        let alpha = 1.0 - beta - gamma;
        let bary = Vector3::new(alpha, beta, gamma);

        let position = self.interpolate_position(bary);
        let mut normal = self.interpolate_normal(bary);

        if normal.dot(&ray.direction) > 0.0 {
            normal = -normal;
        }

        let uv = self.interpolate_uv(bary);

        Some(ObjectHit {
            distance,
            position,
            normal,
            uv,
        })
    }
}
