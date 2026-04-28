use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;

use crate::appearance::Appearance;

pub struct Block {
    pub colour: Rgb,
}

impl Block {
    #[must_use] 
    pub const fn new(colour: Rgb) -> Self {
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
