//! Shader runtime input structure.

pub mod light;
pub mod samples;
pub mod shadow;
pub mod sky;
pub mod sky_builder;

pub use self::{light::*, samples::*, shadow::*, sky::*, sky_builder::*};

use arctk::access;

/// Conglomerate lighting and shadowing settings.
pub struct Shader {
    /// Sky settings.
    sky: Sky,
    /// Lighting samples.
    samples: Samples,
    /// Lighting settings.
    light: Light,
    /// Shadowing settings.
    shadow: Shadow,
}

impl Shader {
    access!(sky, sky_mut, Sky);
    access!(samples, Samples);
    access!(light, Light);
    access!(shadow, Shadow);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sky: Sky, samples: Samples, light: Light, shadow: Shadow) -> Self {
        Self {
            sky,
            samples,
            light,
            shadow,
        }
    }
}
