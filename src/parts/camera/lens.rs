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
