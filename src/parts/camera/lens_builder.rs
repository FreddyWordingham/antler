//! Camera lens implementation.

use arctk::{err::Error, file::Build};
use arctk_attr::input;
use std::path::Path;

/// Lens structure.
#[input]
pub enum LensBuilder {
    /// Perspective projection.
    Perspective {
        /// Horizontal field-of-view [deg].
        fov: f64,
    },
    /// Orthographic projection.
    Orthographic {
        /// Horizontal field-width [m].
        field: f64,
    },
}

impl Build for LensBuilder {
    type Inst = crate::parts::Lens;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Perspective { fov } => Self::Inst::new_perspective(fov.to_degrees()),
            Self::Orthographic { field } => Self::Inst::new_orthographic(field),
        })
    }
}
