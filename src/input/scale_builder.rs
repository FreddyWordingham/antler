//! Window scaling input structure.

use arctk::{err::Error, file::Build};
use arctk_attr::input;
use minifb::Scale;
use std::path::Path;

/// Scale enumeration.
#[input]
pub enum ScaleBuilder {
    X1,
    X2,
    X4,
    X8,
    X16,
    X32,
    FullScreen,
}

impl Build for ScaleBuilder {
    type Inst = Scale;

    ///
    #[inline]
    #[must_use]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        match self {
            Self::X1 => Ok(Self::Inst::X1),
            Self::X2 => Ok(Self::Inst::X2),
            Self::X4 => Ok(Self::Inst::X4),
            Self::X8 => Ok(Self::Inst::X8),
            Self::X16 => Ok(Self::Inst::X16),
            Self::X32 => Ok(Self::Inst::X32),
            Self::FullScreen => Ok(Self::Inst::FitScreen),
        }
    }
}
