//! Loadable shader settings.

use crate::{
    err::Error,
    img::Gradient,
    math::Pos3,
    ord::{Link, Set, X, Y, Z},
    sim::render::Shader,
};
use arctk_attr::load;

/// Colouring settings builder.
#[load]
pub struct ShaderLinker {
    /// Sun position used for lighting calculations [m].
    sun_pos: [f64; 3],
    /// Relative ambient, diffuse, and occlusion lighting powers.
    light: [f64; 3],
    /// Relative ambient and direct shadowing powers.
    shadow: [f64; 2],
    /// Ambient lighting fraction.
    spec_pow: i32,
    /// Lighting and shadowing occlusion testing distances.
    occ_dist: [f64; 2],
    /// Effect fall-off rate.
    fall_off: f64,
    /// Optional number of soft shadowing samples, and angular radius [deg].
    soft_shadow_samples: Option<(i32, f64)>,
    /// Optional number of ambient shadowing samples and the scaling power.
    ambient_shadow_samples: Option<(i32, i32)>,
    /// Sky gradient.
    sky_grad: String,
    /// Data plotting gradient.
    data_grad: String,
}

impl<'a> Link<'a, Gradient> for ShaderLinker {
    type Inst = Shader<'a>;

    #[inline]
    fn requires(&self) -> Vec<String> {
        vec![self.sky_grad.clone(), self.data_grad.clone()]
    }

    #[inline]
    fn link(self, grads: &'a Set<Gradient>) -> Result<Self::Inst, Error> {
        let soft_shadow_samples = if let Some((samples, rad)) = self.soft_shadow_samples {
            Some((samples, rad.to_radians()))
        } else {
            None
        };

        Ok(Self::Inst::new(
            Pos3::new(self.sun_pos[X], self.sun_pos[Y], self.sun_pos[Z]),
            self.light,
            self.shadow,
            self.spec_pow,
            self.occ_dist,
            self.fall_off,
            soft_shadow_samples,
            self.ambient_shadow_samples,
            grads
                .get(&self.sky_grad)
                .unwrap_or_else(|| panic!("Failed to link shader-gradient key: {}", self.sky_grad)),
            grads.get(&self.data_grad).unwrap_or_else(|| {
                panic!("Failed to link shader-gradient key: {}", self.data_grad)
            }),
        ))
    }
}
