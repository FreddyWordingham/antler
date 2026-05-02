use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;
use nalgebra::{Unit, Vector3};

use crate::appearance::Appearance;

pub struct Checkerboard {
    pub size: f32,
    pub colour_a: Rgb,
    pub colour_b: Rgb,
}

impl Checkerboard {
    #[must_use]
    pub const fn new(size: f32, colour_a: Rgb, colour_b: Rgb) -> Self {
        Self {
            size,
            colour_a,
            colour_b,
        }
    }
}

impl Appearance for Checkerboard {
    #[inline]
    fn colour(&self, _direction: &Unit<Vector3<f32>>, contact: &Contact) -> Rgb {
        let position = contact.position;
        if ((position.x / self.size).floor() + (position.y / self.size).floor() + (position.z / self.size).floor())
            as i32
            % 2
            == 0
        {
            self.colour_a
        } else {
            self.colour_b
        }
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
