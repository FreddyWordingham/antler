use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;

use crate::Appearance;

pub struct Wireframe {
    surface_colour: Rgb,
    line_colour: Rgb,
    width: f32,
}

impl Wireframe {
    #[must_use]
    pub const fn new(surface_colour: Rgb, line_colour: Rgb, width: f32) -> Self {
        Self {
            surface_colour,
            line_colour,
            width,
        }
    }
}

impl Appearance for Wireframe {
    #[inline]
    fn emitted(&self, _contact: &Contact) -> Rgb {
        Rgb::BLACK
    }

    #[inline]

    fn shade(&self, _ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb {
        let colour = match contact.barycentric {
            Some(bary) => {
                let edge = bary.x.min(bary.y).min(bary.z);

                if edge <= self.width {
                    self.line_colour
                } else {
                    self.surface_colour
                }
            }

            None => self.surface_colour,
        };

        let n_dot_l = contact.normal.dot(&light.direction).max(0.0);
        colour * light.radiance * n_dot_l
    }
}
