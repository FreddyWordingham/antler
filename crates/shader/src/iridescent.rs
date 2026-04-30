use antler_colour::{Rgb, RgbGradient};
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;

use crate::Appearance;

pub struct Iridescent {
    gradient: RgbGradient,
    power: f32,
}

impl Iridescent {
    #[must_use]
    pub const fn new(gradient: RgbGradient, power: f32) -> Self {
        Self { gradient, power }
    }
}

impl Appearance for Iridescent {
    #[inline]
    fn emitted(&self, _contact: &Contact) -> Rgb {
        Rgb::BLACK
    }

    #[inline]
    fn albedo(&self, _contact: &Contact) -> Rgb {
        Rgb::WHITE
    }

    #[inline]
    fn shade(&self, _ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb {
        let n_dot_l = contact.normal.dot(&light.direction).max(0.0);
        let angle = (1.0 - n_dot_l).powf(self.power);

        self.gradient.sample(angle) * light.radiance * n_dot_l
    }
}
