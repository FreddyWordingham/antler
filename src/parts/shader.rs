//! Shader settings.

use crate::{access, clone, img::Gradient, math::Pos3};

/// Colouring settings.
pub struct Shader<'a> {
    /// Sun position used for lighting calculations [m].
    sun_pos: Pos3,
    /// Ambient, diffuse, and occlusion lighting fractions.
    light: [f64; 3],
    /// Ambient, diffuse, and occlusion shadowing fractions.
    shadow: [f64; 2],
    /// Ambient lighting fraction.
    spec_pow: i32,
    /// Lighting and shadowing occlusion testing distances.
    occ_dist: [f64; 2],
    /// Effect fall-off rate.
    fall_off: f64,
    /// Optional number of soft shadowing samples, and angular radius [rad].
    soft_shadow_samples: Option<(i32, f64)>,
    /// Optional number of ambient shadowing samples and the scaling power.
    ambient_shadow_samples: Option<(i32, i32)>,
    /// Sky colour gradient.
    sky_grad: &'a Gradient,
    /// Data colouring gradient.
    data_grad: &'a Gradient,
}

impl<'a> Shader<'a> {
    access!(sun_pos, Pos3);
    access!(light, [f64; 3]);
    access!(shadow, [f64; 2]);
    clone!(spec_pow, i32);
    access!(occ_dist, [f64; 2]);
    clone!(fall_off, f64);
    clone!(soft_shadow_samples, Option<(i32, f64)>);
    clone!(ambient_shadow_samples, Option<(i32, i32)>);
    access!(sky_grad, Gradient);
    access!(data_grad, Gradient);

    /// Construct a new instance.
    #[allow(clippy::cognitive_complexity)]
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub fn new(
        sun_pos: Pos3,
        light: [f64; 3],
        shadow: [f64; 2],
        spec_pow: i32,
        occ_dist: [f64; 2],
        fall_off: f64,
        soft_shadow_samples: Option<(i32, f64)>,
        ambient_shadow_samples: Option<(i32, i32)>,
        sky_grad: &'a Gradient,
        data_grad: &'a Gradient,
    ) -> Self {
        debug_assert!(light[0] > 0.0);
        debug_assert!(light[1] > 0.0);
        debug_assert!(light[2] > 0.0);
        debug_assert!(shadow[0] > 0.0);
        debug_assert!(shadow[1] > 0.0);
        debug_assert!(spec_pow > 0);
        debug_assert!(occ_dist[0] > 0.0);
        debug_assert!(occ_dist[1] > 0.0);
        debug_assert!(fall_off > 0.0);
        debug_assert!(soft_shadow_samples.is_none() || soft_shadow_samples.unwrap().0 > 1);
        debug_assert!(soft_shadow_samples.is_none() || soft_shadow_samples.unwrap().1 > 0.0);
        debug_assert!(ambient_shadow_samples.is_none() || ambient_shadow_samples.unwrap().0 > 1);
        debug_assert!(ambient_shadow_samples.is_none() || ambient_shadow_samples.unwrap().1 > 0);

        let light_total = light[0] + light[1] + light[2];
        let shadow_total = shadow[0] + shadow[1];

        Self {
            sun_pos,
            light: [
                light[0] / light_total,
                light[1] / light_total,
                light[2] / light_total,
            ],
            shadow: [shadow[0] / shadow_total, shadow[1] / shadow_total],
            spec_pow,
            occ_dist,
            fall_off,
            soft_shadow_samples,
            ambient_shadow_samples,
            sky_grad,
            data_grad,
        }
    }
}
