use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;

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
    fn emitted(&self, _contact: &Contact) -> Rgb {
        Rgb::BLACK
    }

    #[inline]
    fn shade(&self, _ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb {
        let position = contact.position;
        let colour = if ((position.x / self.size).floor()
            + (position.y / self.size).floor()
            + (position.z / self.size).floor()) as i32
            % 2
            == 0
        {
            self.colour_a
        } else {
            self.colour_b
        };

        let n_dot_l = contact.normal.dot(&light.direction).max(0.0);
        colour * light.radiance * n_dot_l
    }
}
