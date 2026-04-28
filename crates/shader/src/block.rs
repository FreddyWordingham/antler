use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};

use crate::{appearance::Appearance, light_sample::LightSample};

pub struct Block {
    pub colour: Rgb,
}

impl Block {
    pub fn new(colour: Rgb) -> Self {
        Self { colour }
    }
}

impl Appearance for Block {
    #[inline]
    fn emitted(&self, _contact: &Contact) -> Rgb {
        Rgb::BLACK
    }

    #[inline]
    fn albedo(&self, _contact: &Contact) -> Rgb {
        self.colour
    }

    #[inline]
    fn shade(&self, _ray: &Ray, contact: &Contact, _light: &LightSample) -> Rgb {
        self.albedo(contact)
    }
}
