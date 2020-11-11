//! Window scaling input structure.

use arctk::{err::Error, file::Build};
use arctk_attr::input;
use minifb::Scale;
use std::path::Path;

/// Scale enumeration.
#[input]
pub enum ScaleBuilder {
    /// No pixel scaling.
    X1,
    /// Double render window pixel size.
    X2,
    /// Quadruple render window pixel size.
    X4,
    /// 8 times render window pixel size.
    X8,
    /// 16 times render window pixel size.
    X16,
    /// 32 times render window pixel size.
    X32,
    /// Fullscreen render window.
    Fullscreen,
}

impl Build for ScaleBuilder {
    type Inst = Scale;

    /// Build a usable instance.
    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        match self {
            Self::X1 => Ok(Self::Inst::X1),
            Self::X2 => Ok(Self::Inst::X2),
            Self::X4 => Ok(Self::Inst::X4),
            Self::X8 => Ok(Self::Inst::X8),
            Self::X16 => Ok(Self::Inst::X16),
            Self::X32 => Ok(Self::Inst::X32),
            Self::Fullscreen => Ok(Self::Inst::FitScreen),
        }
    }
}
