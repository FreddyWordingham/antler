use std::f32::consts::TAU;

use antler_colour::Rgb;
use antler_geometry::Contact;
use nalgebra::{Unit, Vector3};
use rand::{Rng, RngExt};

use crate::{emissive::Emissive, light_sample::LightSample};

pub struct Environment {
    pub zenith: Rgb,
    pub horizon: Rgb,
    pub up: Unit<Vector3<f32>>,
    samples: Option<usize>,
}

impl Environment {
    #[must_use]
    pub const fn new(zenith: Rgb, horizon: Rgb, up: Unit<Vector3<f32>>, samples: Option<usize>) -> Self {
        Self {
            zenith,
            horizon,
            up,
            samples,
        }
    }
}

impl Emissive for Environment {
    fn for_each_sample<R: Rng, F: FnMut(LightSample)>(&self, rng: &mut R, _contact: &Contact, mut f: F) {
        let samples = self.samples.unwrap_or(1).max(1);

        for _ in 0..samples {
            let direction = sample_sphere(rng);
            let t = direction.dot(&self.up).mul_add(0.5, 0.5).clamp(0.0, 1.0);
            let radiance = self.horizon * (1.0 - t) + self.zenith * t;

            f(LightSample {
                direction,
                distance: f32::INFINITY,
                radiance,
            });
        }
    }
}

#[inline]
fn sample_sphere<R: Rng>(rng: &mut R) -> Unit<Vector3<f32>> {
    let z = rng.random_range(-1.0..=1.0);
    let a = TAU * rng.random::<f32>();
    let r = (1.0_f32 - z * z).sqrt();

    Unit::new_unchecked(Vector3::new(r * a.cos(), r * a.sin(), z))
}
