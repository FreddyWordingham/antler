//! Camera focus implementation.

use arctk::{
    access, clone,
    geom::{Orient, Ray},
    math::{rand_circle_point, Dir3, Pos3},
};

/// Focus structure.
#[derive(Debug)]
pub struct Focus {
    /// Orientation.
    orient: Orient,
    /// Target point.
    tar: Pos3,
    /// Optional depth-of-field samples and maximum sampling radius.
    dof: Option<(i32, f64)>,
}

impl Focus {
    access!(orient, Orient);
    access!(tar, Pos3);
    clone!(dof, Option<(i32, f64)>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Pos3, tar: Pos3, dof: Option<(i32, f64)>) -> Self {
        debug_assert!(dof.is_none() || dof.unwrap().0 > 0);
        debug_assert!(dof.is_none() || dof.unwrap().1 > 0.0);

        let tar_dist = nalgebra::distance(&pos, &tar);
        let dof = if let Some((samples, angle)) = dof {
            Some((samples, tar_dist * angle.tan()))
        } else {
            None
        };

        Self {
            orient: Orient::new(Ray::new(pos, Dir3::new_normalize(tar - pos))),
            tar,
            dof,
        }
    }

    /// Calculate the number of depth-of-field samples.
    #[inline]
    #[must_use]
    pub fn dof_samples(&self) -> i32 {
        if let Some((dof_samples, _dof_ang)) = self.dof {
            dof_samples
        } else {
            1
        }
    }

    /// Calculate the nth depth-of-field observation position.
    #[inline]
    #[must_use]
    pub fn observation_pos(&self, offset: f64, n: i32) -> Pos3 {
        let mut pos = *self.orient.pos();

        if let Some((dof_samples, max_rad)) = self.dof {
            let (rho, mut theta) = rand_circle_point(n, dof_samples);
            theta += offset;

            let r = nalgebra::distance(self.orient.pos(), &self.tar);
            pos += self.orient.forward().as_ref() * (r - (r.powi(2) - rho.powi(2)).sqrt());
            pos += self.orient.right().as_ref() * theta.sin() * max_rad * rho;
            pos += self.orient.up().as_ref() * theta.cos() * max_rad * rho;
        }

        pos
    }

    /// Calculate the nth depth-of-field observation ray.
    #[inline]
    #[must_use]
    pub fn observation_ray(&self, offset: f64, n: i32) -> Ray {
        debug_assert!(n >= 0);

        let pos = self.observation_pos(offset, n);
        Ray::new(pos, Dir3::new_normalize(self.tar - pos))
    }
}
