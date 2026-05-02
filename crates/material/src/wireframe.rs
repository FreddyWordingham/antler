use antler_colour::Rgb;
use antler_geometry::{Contact, Ray, utils::offset_origin};
use rand::Rng;

use crate::bsdf::Bsdf;

pub struct Wireframe {
    transparency: f32,
    line_width: f32,
}

impl Wireframe {
    #[must_use] 
    pub const fn new(transparency: f32, line_width: f32) -> Self {
        Self {
            transparency: transparency.clamp(0.0, 1.0),
            line_width,
        }
    }
}

impl Bsdf for Wireframe {
    #[inline]
    fn visibility(&self) -> Rgb {
        Rgb::WHITE * self.transparency
    }

    fn scatter<R: Rng, F: FnMut(Ray, f32)>(
        &self,
        _rng: &mut R,
        ray: &Ray,
        contact: &Contact,
        mut emit_child: F,
    ) -> f32 {
        let is_line = contact
            .barycentric
            .is_some_and(|b| b.x.min(b.y).min(b.z) <= self.line_width);

        if is_line {
            return 1.0;
        }

        if self.transparency > 0.0 {
            emit_child(
                Ray {
                    origin: offset_origin(contact.position, -contact.normal, ray.direction),
                    direction: ray.direction,
                },
                self.transparency,
            );
        }

        1.0 - self.transparency
    }
}
