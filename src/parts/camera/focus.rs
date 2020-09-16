//! Camera focus implementation.

use arctk::{
    access,
    geom::{Orient, Ray},
    math::{Dir3, Pos3},
};

/// Focus structure.
#[derive(Debug)]
pub struct Focus {
    /// Orientation.
    orient: Orient,
    /// Target point.
    tar: Pos3,
}

impl Focus {
    access!(orient, orient_mut, Orient);
    access!(tar, Pos3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Pos3, tar: Pos3) -> Self {
        Self {
            orient: Orient::new(Ray::new(pos, Dir3::new_normalize(tar - pos))),
            tar,
        }
    }

    /// Calculate the nth depth-of-field observation ray.
    #[inline]
    #[must_use]
    pub fn observation_ray(&self) -> Ray {
        Ray::new(
            *self.orient.pos(),
            Dir3::new_normalize(self.tar - self.orient.pos()),
        )
    }
}
