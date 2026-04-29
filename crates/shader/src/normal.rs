use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;

use crate::appearance::Appearance;

pub struct Normal;

impl Normal {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Appearance for Normal {
    #[inline]
    fn emitted(&self, _contact: &Contact) -> Rgb {
        Rgb::BLACK
    }

    #[inline]
    fn albedo(&self, contact: &Contact) -> Rgb {
        Rgb::new(contact.normal.x.abs(), contact.normal.y.abs(), contact.normal.z.abs())
    }

    #[inline]
    fn shade(&self, _ray: &Ray, contact: &Contact, _light: &LightSample) -> Rgb {
        self.albedo(contact)
    }
}
