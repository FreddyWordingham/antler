use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;

use crate::appearance::Appearance;

pub struct Luminous {
    pub colour: Rgb,
    pub intensity: f32,
}

impl Luminous {
    #[must_use]
    pub const fn new(colour: Rgb, intensity: f32) -> Self {
        Self { colour, intensity }
    }
}

impl Appearance for Luminous {
    #[inline]
    fn emitted(&self, _contact: &Contact) -> Rgb {
        self.colour * self.intensity
    }

    #[inline]
    fn shade(&self, _ray: &Ray, _contact: &Contact, _light: &LightSample) -> Rgb {
        Rgb::BLACK
    }
}
