use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;
use nalgebra::{Unit, Vector3};

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
    fn colour(&self, _direction: &Unit<Vector3<f32>>, contact: &Contact) -> Rgb {
        match contact.barycentric {
            Some(bary) => {
                let edge = bary.x.min(bary.y).min(bary.z);

                if edge <= self.width {
                    self.line_colour
                } else {
                    self.surface_colour
                }
            }

            None => self.surface_colour,
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
