use antler_colour::Rgb;
use antler_geometry::Contact;
use nalgebra::{Unit, Vector3};
use rand::Rng;

use crate::{emissive::Emissive, light_sample::LightSample, utils::cone_direction};

pub struct Directional {
    pub direction: Unit<Vector3<f32>>,
    pub colour: Rgb,
    angular_radius: Option<f32>,
    samples: Option<usize>,
}

impl Directional {
    #[must_use]
    pub const fn new(
        direction: Unit<Vector3<f32>>,
        colour: Rgb,
        angular_radius: Option<f32>,
        samples: Option<usize>,
    ) -> Self {
        Self {
            direction,
            colour,
            angular_radius,
            samples,
        }
    }
}

impl Emissive for Directional {
    fn for_each_sample<R: Rng, F: FnMut(LightSample)>(&self, rng: &mut R, _contact: &Contact, mut f: F) {
        let angular_radius = self.angular_radius.unwrap_or(0.0);
        let samples = self.samples.unwrap_or(1).max(1);

        if angular_radius <= 0.0 || samples == 1 {
            f(LightSample {
                direction: -self.direction,
                distance: f32::INFINITY,
                radiance: self.colour,
            });
            return;
        }

        for _ in 0..samples {
            f(LightSample {
                direction: cone_direction(-self.direction, angular_radius, rng),
                distance: f32::INFINITY,
                radiance: self.colour,
            });
        }
    }
}
