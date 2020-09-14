//! Shader form.

use crate::{
    input::{Light, Samples, Shader, Shadow, SkyBuilder},
    parts::CameraBuilder,
};
use arctk::{access, err::Error, file::Build};
use arctk_attr::input;
use std::path::Path;

/// Shader settings.
#[input]
pub struct ShaderBuilder {
    /// Sky builder.
    sky: SkyBuilder,
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
    access!(sky, SkyBuilder);
    access!(samples, Samples);
    access!(light, Light);
    access!(shadow, Shadow);
    access!(cam, CameraBuilder);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        sky: SkyBuilder,
        samples: Samples,
        light: Light,
        shadow: Shadow,
        cam: CameraBuilder,
    ) -> Self {
        Self {
            sky,
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
            self.sky.build(in_dir)?,
            self.samples,
            self.light,
            self.shadow,
            self.cam.build(in_dir)?,
        ))
    }
}
