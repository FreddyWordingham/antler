use std::f32::{INFINITY, NEG_INFINITY};

use nalgebra::{Point2, Point3, Similarity3, Vector3};

use crate::{
    geometry::{Bounded, Traceable},
    tracing::{ObjectHit, ObjectRay},
};

const FACE_EPSILON: f32 = 1.0e-4;
const PARALLEL_THRESHOLD: f32 = 1e-8;

#[derive(Debug, Clone)]
pub struct Aabb {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl Aabb {
    pub fn new(min: Point3<f32>, max: Point3<f32>) -> Self {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        assert!(min.z <= max.z);
        Self { min, max }
    }

    pub fn union<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let Some(first) = iter.next() else {
            panic!("Cannot union an empty iterator of AABBs.");
        };

        iter.fold(first, |a, b| Self {
            min: Point3::new(a.min.x.min(b.min.x), a.min.y.min(b.min.y), a.min.z.min(b.min.z)),
            max: Point3::new(a.max.x.max(b.max.x), a.max.y.max(b.max.y), a.max.z.max(b.max.z)),
        })
    }

    pub fn vertices(&self) -> [Point3<f32>; 8] {
        [
            self.min,
            Point3::new(self.max.x, self.min.y, self.min.z),
            Point3::new(self.min.x, self.max.y, self.min.z),
            Point3::new(self.max.x, self.max.y, self.min.z),
            Point3::new(self.min.x, self.min.y, self.max.z),
            Point3::new(self.max.x, self.min.y, self.max.z),
            Point3::new(self.min.x, self.max.y, self.max.z),
            self.max,
        ]
    }

    pub fn transform(&self, transform: &Similarity3<f32>) -> Self {
        let transformed_vertices = self.vertices().into_iter().map(|v| transform.transform_point(&v));
        let min = transformed_vertices
            .clone()
            .fold(Point3::new(INFINITY, INFINITY, INFINITY), |a, b| {
                Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
            });
        let max = transformed_vertices.fold(Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY), |a, b| {
            Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
        });
        Self { min, max }
    }

    #[inline]
    pub fn centroid(&self) -> Point3<f32> {
        (self.min + self.max.coords) / 2.0
    }

    pub fn ray_interval(&self, ray: &crate::geometry::Ray) -> Option<(f32, f32)> {
        let origin = ray.origin;
        let direction = ray.direction.into_inner();

        let mut t_min = NEG_INFINITY;
        let mut t_max = INFINITY;

        for axis in 0..3 {
            let o = origin[axis];
            let d = direction[axis];
            let min = self.min[axis];
            let max = self.max[axis];

            if d.abs() < PARALLEL_THRESHOLD {
                if o < min || o > max {
                    return None;
                }
                continue;
            }

            let inv_d = 1.0 / d;
            let mut t0 = (min - o) * inv_d;
            let mut t1 = (max - o) * inv_d;

            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max < t_min {
                return None;
            }
        }

        Some((t_min, t_max))
    }
}

impl Bounded for Aabb {
    fn bounds(&self) -> Aabb {
        self.clone()
    }
}

impl Traceable for Aabb {
    fn trace(&self, ray: &ObjectRay) -> Option<ObjectHit> {
        let (t_min, t_max) = self.ray_interval(ray)?;

        let distance = if t_min > 0.0 { t_min } else { t_max };
        if distance <= 0.0 {
            return None;
        }

        let position = ray.origin + *ray.direction * distance;

        let normal = if (position.x - self.min.x).abs() < FACE_EPSILON {
            -Vector3::x_axis()
        } else if (position.x - self.max.x).abs() < FACE_EPSILON {
            Vector3::x_axis()
        } else if (position.y - self.min.y).abs() < FACE_EPSILON {
            -Vector3::y_axis()
        } else if (position.y - self.max.y).abs() < FACE_EPSILON {
            Vector3::y_axis()
        } else if (position.z - self.min.z).abs() < FACE_EPSILON {
            -Vector3::z_axis()
        } else {
            Vector3::z_axis()
        };

        Some(ObjectHit {
            distance,
            position,
            normal,
            uv: Point2::new(0.0, 0.0),
        })
    }
}
