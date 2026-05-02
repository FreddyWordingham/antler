use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;
use nalgebra::{Unit, Vector3};

use crate::Appearance;

pub struct Solid {
    colour: Rgb,
}

impl Solid {
    #[must_use]
    pub const fn new(colour: Rgb) -> Self {
        Self { colour }
    }
}

impl Appearance for Solid {
    #[inline]
    fn colour(&self, _direction: &Unit<Vector3<f32>>, _contact: &Contact) -> Rgb {
        self.colour
    }

    #[inline]
    fn emitted(&self, _contact: &Contact) -> Rgb {
        Rgb::BLACK
    }

    #[inline]
    fn shade(&self, ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb {
        let n_dot_l = contact.normal.dot(&light.direction).max(0.0);
        self.colour(&ray.direction, contact) * light.radiance * n_dot_l
    }
}
