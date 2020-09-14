//! Camera lens implementation.

use arctk::clone;

/// Lens structure.
#[derive(Debug)]
pub struct Lens {
    /// Swivel to apply after targeting [rad].
    swivel: [f64; 2],
    /// Field of view [rad].
    fov: f64,
}

impl Lens {
    clone!(swivel, [f64; 2]);
    clone!(fov, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(swivel: [f64; 2], fov: f64) -> Self {
        debug_assert!(fov > 0.0);

        Self { swivel, fov }
    }
}
