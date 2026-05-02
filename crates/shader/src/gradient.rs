use antler_colour::{Rgb, RgbGradient};
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;
use nalgebra::{Unit, Vector3};

use crate::Appearance;

pub struct Gradient {
    gradient: RgbGradient,
    power: f32,
}

impl Gradient {
    #[must_use]
    pub const fn new(gradient: RgbGradient, power: f32) -> Self {
        Self { gradient, power }
    }
}

impl Appearance for Gradient {
    #[inline]
    fn colour(&self, direction: &Unit<Vector3<f32>>, contact: &Contact) -> Rgb {
        let n_dot_l = contact.normal.dot(direction).clamp(0.0, 1.0);
        let angle = (1.0 - n_dot_l).powf(self.power);
        self.gradient.sample(angle)
    }

    #[inline]
    fn emitted(&self, _contact: &Contact) -> Rgb {
        Rgb::BLACK
    }

    #[inline]
    fn shade(&self, _ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb {
        let n_dot_l = contact.normal.dot(&light.direction).clamp(0.0, 1.0);
        self.colour(&light.direction, contact) * light.radiance * n_dot_l
    }
}
