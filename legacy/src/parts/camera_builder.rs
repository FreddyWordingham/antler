//! Camera-builder implementation.

use crate::parts::{Focus, LensBuilder, Sensor};
use arctk::{err::Error, file::Build, img::AspectRatio, math::Pos3};
use arctk_attr::input;
use std::path::Path;

/// Loadable camera builder structure.
#[input]
pub struct CameraBuilder {
    /// Camera position [m].
    pos: Pos3,
    /// Target position [m].
    tar: Pos3,
    /// Lens choice.
    lens: LensBuilder,
    /// Aspect ratio.
    aspect: AspectRatio,
    /// Horizontal pixel resolution.
    hr_res: u64,
    /// Optional super-sampling power.
    ss_power: Option<i32>,
}

impl Build for CameraBuilder {
    type Inst = crate::parts::Camera;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let focus = Focus::new(self.pos, self.tar);
        let lens = self.lens.build(in_dir)?;
        let sensor = Sensor::new(&self.aspect, self.hr_res, self.ss_power);

        Ok(Self::Inst::new(focus, lens, sensor))
    }
}
