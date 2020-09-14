//! Light setup structure.

use arctk::clone;
use arctk_attr::input;

/// Lighting structure.
#[input]
pub struct Light {
    /// Ambient lighting fraction.
    ambient: f64,
    /// Diffuse lighting fraction.
    diffuse: f64,
    /// Specular lighting fraction.
    specular: f64,
    /// Specular lighting power.
    spec_pow: i32,
}

impl Light {
    clone!(ambient, f64);
    clone!(diffuse, f64);
    clone!(specular, f64);
    clone!(spec_pow, i32);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(ambient: f64, diffuse: f64, specular: f64, spec_pow: i32) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            spec_pow,
        }
    }
}
