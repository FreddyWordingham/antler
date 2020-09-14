//! Shader runtime input structure.

pub mod light;
pub mod samples;
pub mod shadow;

pub use self::{light::*, samples::*, shadow::*};

use crate::render::Camera;
use arctk::access;

/// Conglomerate lighting and shadowing settings.
pub struct Shader {
    /// Lighting samples.
    samples: Samples,
    /// Lighting settings.
    light: Light,
    /// Shadowing settings.
    shadow: Shadow,
    /// Imaging camera.
    cam: Camera,
}

impl Shader {
    access!(samples, Samples);
    access!(light, Light);
    access!(shadow, Shadow);
    access!(cam, Camera);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(samples: Samples, light: Light, shadow: Shadow, cam: Camera) -> Self {
        Self {
            samples,
            light,
            shadow,
            cam,
        }
    }
}
