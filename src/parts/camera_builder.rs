//! Camera-builder implementation.

use crate::parts::{Focus, Lens, Sensor};
use arctk::{
    err::Error,
    file::Build,
    img::AspectRatio,
    math::Pos3,
    ord::{X, Y},
};
use arctk_attr::input;
use std::path::Path;

/// Loadable camera builder structure.
#[input]
pub struct CameraBuilder {
    /// Camera position [m].
    pos: Pos3,
    /// Target position [m].
    tar: Pos3,
    /// Optional targeting swivel adjustment [deg].
    swivel: Option<[f64; 2]>,
    /// Horizontal field of view [deg].
    hr_fov: f64,
    /// Aspect ratio.
    aspect: AspectRatio,
    /// Horizontal pixel resolution.
    hr_res: u64,
    /// Optional super-sampling power.
    ss_power: Option<i32>,
}

impl Build for CameraBuilder {
    type Inst = super::Camera;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        let focus = Focus::new(self.pos, self.tar);

        let swivel = if let Some(s) = self.swivel {
            s
        } else {
            [0.0, 0.0]
        };
        let lens = Lens::new_perspective(self.hr_fov.to_radians());
        let sensor = Sensor::new(&self.aspect, self.hr_res, self.ss_power);

        Ok(Self::Inst::new(focus, lens, sensor))
    }
}
