use antler_geometry::{Contact, Ray, utils::offset_origin};
use rand::Rng;

use crate::bsdf::Bsdf;

pub struct Wireframe {
    surface_alpha: f32,
    line_width: f32,
}

impl Wireframe {
    pub const fn new(surface_alpha: f32, line_width: f32) -> Self {
        Self {
            surface_alpha,
            line_width,
        }
    }
}

impl Bsdf for Wireframe {
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

        let surface_alpha = self.surface_alpha.clamp(0.0, 1.0);

        if surface_alpha < 1.0 {
            emit_child(
                Ray {
                    origin: offset_origin(contact.position, -contact.normal, ray.direction),
                    direction: ray.direction,
                },
                1.0 - surface_alpha,
            );
        }

        surface_alpha
    }
}
