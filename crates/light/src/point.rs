use antler_colour::Rgb;
use antler_geometry::Contact;
use nalgebra::Point3;
use rand::Rng;

use crate::{emissive::Emissive, light_sample::LightSample, utils::cone_direction};

pub struct Point {
    pub position: Point3<f32>,
    pub colour: Rgb,
    pub intensity: f32,
    angular_radius: Option<f32>,
    samples: Option<usize>,
}

impl Point {
    #[must_use]
    pub fn new(
        position: Point3<f32>,
        colour: Rgb,
        intensity: f32,
        angular_radius: Option<f32>,
        samples: Option<usize>,
    ) -> Self {
        Self {
            position,
            colour,
            intensity: intensity.max(0.0),
            angular_radius,
            samples,
        }
    }
}

impl Emissive for Point {
    fn for_each_sample<R: Rng, F: FnMut(LightSample)>(&self, rng: &mut R, contact: &Contact, mut f: F) {
        let to_light = self.position - contact.position;
        let distance_squared = to_light.norm_squared();

        if distance_squared <= 1.0e-8 {
            return;
        }

        let distance = distance_squared.sqrt();
        let base_dir = nalgebra::Unit::new_normalize(to_light);

        let angular_radius = self.angular_radius.unwrap_or(0.0);
        let samples = self.samples.unwrap_or(1).max(1);

        let radiance = self.colour * (self.intensity / distance_squared);

        if angular_radius <= 0.0 || samples == 1 {
            f(LightSample {
                direction: base_dir,
                distance,
                radiance,
            });
            return;
        }

        for _ in 0..samples {
            let direction = cone_direction(base_dir, angular_radius, rng);

            f(LightSample {
                direction,
                distance,
                radiance,
            });
        }
    }
}
