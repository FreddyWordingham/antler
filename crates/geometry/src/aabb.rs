use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{bounded::Bounded, hit::Hit, ray::Ray, traceable::Traceable};

const PARALLEL_THRESHOLD: f32 = 1e-8;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    min: Point3<f32>,
    max: Point3<f32>,
}

impl Aabb {
    #[must_use]
    pub fn new(min: Point3<f32>, max: Point3<f32>) -> Self {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        assert!(min.z <= max.z);
        Self { min, max }
    }

    #[must_use]
    pub fn ray_intersection(&self, ray: &Ray) -> Option<(f32, f32, AabbFace, AabbFace)> {
        let origin = ray.origin;
        let direction = ray.direction.into_inner();

        let mut t_min = f32::NEG_INFINITY;
        let mut t_max = f32::INFINITY;

        let mut entry_face = AabbFace::MinX;
        let mut exit_face = AabbFace::MaxX;

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

            let (face0, face1) = match axis {
                0 => (AabbFace::MinX, AabbFace::MaxX),
                1 => (AabbFace::MinY, AabbFace::MaxY),
                _ => (AabbFace::MinZ, AabbFace::MaxZ),
            };

            let (near_face, far_face) = if t0 <= t1 {
                (face0, face1)
            } else {
                std::mem::swap(&mut t0, &mut t1);
                (face1, face0)
            };

            if t0 > t_min {
                t_min = t0;
                entry_face = near_face;
            }

            if t1 < t_max {
                t_max = t1;
                exit_face = far_face;
            }

            if t_max < t_min {
                return None;
            }
        }

        Some((t_min, t_max, entry_face, exit_face))
    }
}

impl Bounded for Aabb {
    fn bounds(&self) -> Aabb {
        *self
    }
}

impl Traceable for Aabb {
    #[inline]
    fn distance(&self, ray: &Ray) -> Option<f32> {
        let (t_min, t_max, _, _) = self.ray_intersection(ray)?;
        let distance = if t_min > 0.0 { t_min } else { t_max };
        (distance > 0.0).then_some(distance)
    }

    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let (t_min, t_max, entry_face, exit_face) = self.ray_intersection(ray)?;

        let (distance, normal) = if t_min > 0.0 {
            (t_min, entry_face.normal())
        } else {
            (t_max, exit_face.normal())
        };

        if distance <= 0.0 {
            return None;
        }

        let position = ray.origin + *ray.direction * distance;

        Some(Hit::new(distance, position, normal, Point2::new(0.0, 0.0)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum AabbFace {
    MinX,
    MaxX,
    MinY,
    MaxY,
    MinZ,
    MaxZ,
}

impl AabbFace {
    #[inline]
    pub fn normal(self) -> Unit<Vector3<f32>> {
        match self {
            Self::MinX => -Vector3::x_axis(),
            Self::MaxX => Vector3::x_axis(),
            Self::MinY => -Vector3::y_axis(),
            Self::MaxY => Vector3::y_axis(),
            Self::MinZ => -Vector3::z_axis(),
            Self::MaxZ => Vector3::z_axis(),
        }
    }
}
