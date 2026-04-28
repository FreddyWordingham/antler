use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};

use crate::{Appearance, light_sample::LightSample};

pub struct Solid {
    pub colour: Rgb,
}

impl Solid {
    pub fn new(colour: Rgb) -> Self {
        Self { colour }
    }
}

impl Appearance for Solid {
    #[inline]
    fn emitted(&self, _contact: &Contact) -> Rgb {
        Rgb::BLACK
    }

    #[inline]
    fn albedo(&self, _contact: &Contact) -> Rgb {
        self.colour
    }

    #[inline]
    fn shade(&self, _ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb {
        let n_dot_l = contact.normal.dot(&light.direction).max(0.0);
        self.albedo(contact) * light.radiance * n_dot_l
    }
}
