//! Shader form.

use crate::input::{Light, Samples, Shader, Shadow};
use crate::parts::CameraBuilder;
use arctk::{access, err::Error, file::Build};
use arctk_attr::input;
use std::path::Path;

/// Loadable light and shadow settings.
#[input]
pub struct ShaderBuilder {
    /// Lighting samples.
    samples: Samples,
    /// Lighting settings.
    light: Light,
    /// Shadowing settings.
    shadow: Shadow,
    /// Camera builder.
    cam: CameraBuilder,
}

impl ShaderBuilder {
    access!(samples, Samples);
    access!(light, Light);
    access!(shadow, Shadow);
    access!(cam, CameraBuilder);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(samples: Samples, light: Light, shadow: Shadow, cam: CameraBuilder) -> Self {
        Self {
            samples,
            light,
            shadow,
            cam,
        }
    }
}

impl Build for ShaderBuilder {
    type Inst = Shader;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new(
            self.samples,
            self.light,
            self.shadow,
            self.cam.build(in_dir)?,
        ))
    }
}
