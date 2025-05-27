//! Axis-aligned bounding box structure.

use nalgebra::{Point3, RealField};

use crate::geometry::Ray;

/// Axis-aligned bounding box.
#[derive(Debug, Clone)]
pub struct Aabb<T: RealField> {
    /// Minimum corner [x, y, z].
    pub mins: Point3<T>,
    /// Maximum corner [x, y, z].
    pub maxs: Point3<T>,
}

impl<T: RealField> Aabb<T> {
    /// Construct a new `Aabb` instance.
    pub fn new(mins: Point3<T>, maxs: Point3<T>) -> Self {
        debug_assert!(
            mins <= maxs,
            "Axis-aligned bounding box minimums must be less than, or equal to, the maximums!"
        );

        Self { mins, maxs }
    }

    /// Construct a new `Aabb` instance without checking the minimums are less than, or equal to, the maximums.
    #[must_use]
    #[inline]
    pub const fn new_unchecked(mins: Point3<T>, maxs: Point3<T>) -> Self {
        Self { mins, maxs }
    }

    /// Get the center of the `Aabb`.
    #[must_use]
    #[inline]
    pub fn centre(&self) -> Point3<T> {
        Point3::new(
            (self.mins.x.clone() + self.maxs.x.clone()) * T::from_f32(0.5).unwrap(),
            (self.mins.y.clone() + self.maxs.y.clone()) * T::from_f32(0.5).unwrap(),
            (self.mins.z.clone() + self.maxs.z.clone()) * T::from_f32(0.5).unwrap(),
        )
    }

    /// Find the union of two axis-aligned bounding boxes.
    #[must_use]
    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        let mins = Point3::new(
            self.mins.x.clone().min(other.mins.x.clone()),
            self.mins.y.clone().min(other.mins.y.clone()),
            self.mins.z.clone().min(other.mins.z.clone()),
        );
        let maxs = Point3::new(
            self.maxs.x.clone().max(other.maxs.x.clone()),
            self.maxs.y.clone().max(other.maxs.y.clone()),
            self.maxs.z.clone().max(other.maxs.z.clone()),
        );
        Self::new(mins, maxs)
    }

    /// Test for an intersection between a `Ray` and the `Aabb`.
    pub fn intersect_distance(&self, ray: &Ray<T>) -> Option<T> {
        let mut t_min = T::zero();
        let mut t_max = T::max_value().unwrap();

        for i in 0..3 {
            let ray_origin_i = ray.origin[i].clone();
            let ray_dir_i = ray.direction[i].clone();
            let box_min_i = self.mins[i].clone();
            let box_max_i = self.maxs[i].clone();

            if ray_dir_i.clone().abs() < T::default_epsilon() {
                // Ray is parallel to the slab
                if ray_origin_i < box_min_i || ray_origin_i > box_max_i {
                    return None;
                }
            } else {
                let inv_dir = T::one() / ray_dir_i;
                let mut t0 = (box_min_i - ray_origin_i.clone()) * inv_dir.clone();
                let mut t1 = (box_max_i - ray_origin_i) * inv_dir;

                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1);
                }

                t_min = t_min.max(t0);
                t_max = t_max.min(t1);

                if t_min > t_max {
                    return None;
                }
            }
        }

        if t_max < T::zero() {
            return None;
        }

        Some(if t_min >= T::zero() { t_min } else { t_max })
    }
}
