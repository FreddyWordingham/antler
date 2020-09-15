//! Camera lens implementation.

/// Lens structure.
#[derive(Debug)]
pub enum Lens {
    /// Perspective projection.
    Perspective {
        /// Horizontal field-of-view [rad].
        fov: f64,
    },
}

impl Lens {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new_perspective(fov: f64) -> Self {
        debug_assert!(fov > 0.0);

        Self::Perspective { fov }
    }
}
