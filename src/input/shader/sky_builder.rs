//! Sky properties.

use crate::input::Sky;
use arctk::{err::Error, file::Build, math::Pos3};
use arctk_attr::input;
use std::path::Path;

/// Scene properties.
#[input]
pub struct SkyBuilder {
    /// Sky brightness fraction.
    brightness: f64,
    /// Sun position when calculating direct shadows [m].
    sun_pos: Pos3,
    /// Sun angular radius when calculating soft shadows [deg].
    sun_rad: f64,
}

impl Build for SkyBuilder {
    type Inst = Sky;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new(
            self.brightness,
            self.sun_pos,
            self.sun_rad.to_radians(),
        ))
    }
}
