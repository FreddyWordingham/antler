use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;

use crate::appearance::Appearance;

pub struct Luminous {
    pub colour: Rgb,
    pub intensity: f32,
}

impl Luminous {
    pub fn new(colour: Rgb, intensity: f32) -> Self {
        Self { colour, intensity }
    }
}

impl Appearance for Luminous {
    #[inline]
    fn emitted(&self, contact: &Contact) -> Rgb {
        self.albedo(contact) * self.intensity
    }

    #[inline]
    fn albedo(&self, _contact: &Contact) -> Rgb {
        self.colour
    }

    #[inline]
    fn shade(&self, _ray: &Ray, _contact: &Contact, _light: &LightSample) -> Rgb {
        Rgb::BLACK
    }
}
