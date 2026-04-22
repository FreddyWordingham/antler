use std::f32::{EPSILON, INFINITY, NEG_INFINITY};

use nalgebra::{Point2, Point3, Similarity3, Vector3};

use crate::{
    geometry::{Bounded, Ray, Traceable},
    tracing::Hit,
};

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
}

impl Bounded for Aabb {
    fn bounds(&self) -> Aabb {
        self.clone()
    }
}

impl Traceable for Aabb {
    fn trace(&self, ray: &Ray) -> Option<Hit> {
        let origin = ray.origin;
        let direction = ray.direction.into_inner();

        let mut t_min = NEG_INFINITY;
        let mut t_max = INFINITY;

        for axis in 0..3 {
            let o = origin[axis];
            let d = direction[axis];
            let min = self.min[axis];
            let max = self.max[axis];

            if d.abs() < EPSILON {
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

        let distance = if t_min > 0.0 { t_min } else { t_max };
        if distance <= 0.0 {
            return None;
        }

        let position = ray.origin + *ray.direction * distance;

        let normal = if (position.x - self.min.x).abs() < EPSILON {
            -Vector3::x_axis()
        } else if (position.x - self.max.x).abs() < EPSILON {
            Vector3::x_axis()
        } else if (position.y - self.min.y).abs() < EPSILON {
            -Vector3::y_axis()
        } else if (position.y - self.max.y).abs() < EPSILON {
            Vector3::y_axis()
        } else if (position.z - self.min.z).abs() < EPSILON {
            -Vector3::z_axis()
        } else {
            Vector3::z_axis()
        };

        Some(Hit {
            distance,
            position,
            normal,
            uv: Point2::new(0.0, 0.0),
        })
    }
}
