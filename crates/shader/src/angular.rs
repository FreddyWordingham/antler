use antler_colour::{Rgb, RgbGradient};
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;
use nalgebra::{Unit, Vector3};

use crate::Appearance;

pub struct Angular {
    gradient: RgbGradient,
    power: f32,
    direction: Unit<Vector3<f32>>,
}

impl Angular {
    #[must_use]
    pub const fn new(gradient: RgbGradient, power: f32, direction: Unit<Vector3<f32>>) -> Self {
        Self {
            gradient,
            power,
            direction,
        }
    }
}

impl Appearance for Angular {
    #[inline]
    fn emitted(&self, _contact: &Contact) -> Rgb {
        Rgb::BLACK
    }

    #[inline]
    fn shade(&self, _ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb {
        let n_dot_d = contact.normal.dot(&self.direction);
        let t = (n_dot_d * 0.5 + 0.5).clamp(0.0, 1.0).powf(self.power);

        let n_dot_l = contact.normal.dot(&light.direction).max(0.0);

        self.gradient.sample(t) * light.radiance * n_dot_l
    }
}
