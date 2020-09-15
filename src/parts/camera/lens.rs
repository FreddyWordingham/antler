//! Camera lens implementation.

/// Lens structure.
#[derive(Debug)]
pub enum Lens {
    /// Perspective projection.
    Perspective {
        /// Horizontal field-of-view [rad].
        fov: f64,
    },
    /// Orthographic projection.
    Orthographic {
        /// Horizontal field-width [m].
        field: f64,
    },
}

impl Lens {
    /// Construct a new perspective instance.
    #[inline]
    #[must_use]
    pub fn new_perspective(fov: f64) -> Self {
        debug_assert!(fov > 0.0);

        Self::Perspective { fov }
    }

    /// Construct a new orthographic instance.
    #[inline]
    #[must_use]
    pub fn new_orthographic(field: f64) -> Self {
        debug_assert!(field > 0.0);

        Self::Orthographic { field }
    }
}
