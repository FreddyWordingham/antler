//! Shadow setup structure.

use arctk::{access, clone};
use arctk_attr::input;

/// Lighting structure.
#[input]
pub struct Shadow {
    /// Ambient shadowing fraction.
    ambient: f64,
    /// Direct shadowing fraction.
    direct: f64,
    /// Ambient occlusion power.
    ao_pow: i32,
}

impl Shadow {
    access!(ambient, f64);
    access!(direct, f64);
    clone!(ao_pow, i32);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(ambient: f64, direct: f64, ao_pow: i32) -> Self {
        Self {
            ambient,
            direct,
            ao_pow,
        }
    }
}
