//! Camera structure.

pub mod focus;
pub mod lens;
pub mod lens_builder;
pub mod sensor;

pub use self::{focus::*, lens::*, lens_builder::*, sensor::*};

use arctk::{
    access,
    geom::Ray,
    math::{Dir3, Pos3, Rot3},
    ord::{X, Y},
};

/// Camera structure.
#[derive(Debug)]
pub struct Camera {
    /// Focus.
    focus: Focus,
    /// Lens.
    lens: Lens,
    /// Sensor.
    sensor: Sensor,
}

impl Camera {
    access!(focus, focus_mut, Focus);
    access!(lens, Lens);
    access!(sensor, Sensor);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(focus: Focus, lens: Lens, sensor: Sensor) -> Self {
        Self {
            focus,
            lens,
            sensor,
        }
    }

    /// Reference the forward direction.
    #[inline]
    #[must_use]
    pub const fn forward(&self) -> &Dir3 {
        self.focus.orient().forward()
    }

    /// Reference the upwards direction.
    #[inline]
    #[must_use]
    pub const fn up(&self) -> &Dir3 {
        self.focus.orient().up()
    }

    /// Reference the right direction.
    #[inline]
    #[must_use]
    pub const fn right(&self) -> &Dir3 {
        self.focus.orient().right()
    }

    /// Set a new camera position.
    #[inline]
    pub fn set_pos(&mut self, pos: Pos3) {
        self.focus = Focus::new(pos, *self.focus.tar());
    }

    /// Set a new target position.
    #[inline]
    pub fn set_tar(&mut self, tar: Pos3) {
        self.focus = Focus::new(*self.focus.orient().pos(), tar);
    }

    /// Generate a new observation ray.
    #[inline]
    #[must_use]
    pub fn gen_ray(&self, pixel: [usize; 2], sub_sample: i32) -> Ray {
        let mut ray = self.focus.observation_ray();

        match self.lens {
            Lens::Perspective { fov } => {
                let delta = fov / (self.sensor.res().0 - 1) as f64;

                let mut theta = ((pixel[X] as f64) * delta) - (fov * 0.5);
                let mut phi = ((pixel[Y] as f64) * delta)
                    - (fov * 0.5 * (self.sensor.res().1 as f64 / self.sensor.res().0 as f64));

                if let Some(super_sample_power) = self.sensor.super_sample_power() {
                    let sub_delta = delta / f64::from(super_sample_power);
                    let sx = f64::from(sub_sample % super_sample_power) + 0.5;
                    let sy = f64::from(sub_sample / super_sample_power) + 0.5;
                    theta += (sub_delta * (0.5 + sx)) - (delta * 0.5);
                    phi += (sub_delta * (0.5 + sy)) - (delta * 0.5);
                }

                *ray.dir_mut() = Rot3::from_axis_angle(&self.focus.orient().down(), theta)
                    * Rot3::from_axis_angle(self.focus.orient().right(), phi)
                    * ray.dir();
            }
            Lens::Orthographic { field } => {
                let w = field;
                let h = (field * self.sensor.res().1 as f64) / self.sensor.res().0 as f64;

                let ss = self.sensor.super_samples();

                let dw = w / ((self.sensor.res().0 * ss as u64) - 1) as f64;
                let dh = h / ((self.sensor.res().1 * ss as u64) - 1) as f64;

                let sx = sub_sample % ss;
                let sy = sub_sample / ss;

                let dx = (dw * ((pixel[X] * ss as usize) + sx as usize) as f64) - (w / 2.0);
                let dy = (dh * ((pixel[Y] * ss as usize) + sy as usize) as f64) - (h / 2.0);

                *ray.pos_mut() += (dx * self.right().as_ref()) + (dy * self.up().as_ref());
            }
        }

        ray
    }
}
