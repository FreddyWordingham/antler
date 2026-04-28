use antler_geometry::{Contact, Ray};
use rand::Rng;

use crate::{
    bsdf::Bsdf,
    utils::{offset_origin, reflect, refract, schlick},
};

pub struct Refractive {
    refractive_index: f32,
}

impl Refractive {
    pub fn new(refractive_index: f32) -> Self {
        Self {
            refractive_index: refractive_index.max(f32::EPSILON),
        }
    }
}

impl Bsdf for Refractive {
    fn scatter<R: Rng, F: FnMut(Ray, f32)>(
        &self,
        _rng: &mut R,
        ray: &Ray,
        contact: &Contact,
        mut emit_child: F,
    ) -> f32 {
        let incident = ray.direction;
        let outward_normal = contact.normal;

        let front_face = incident.dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        let eta = if front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let cos_theta = (-incident).dot(&normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = eta * sin_theta > 1.0;
        let reflectance = if cannot_refract { 1.0 } else { schlick(cos_theta, eta) };

        let reflected = reflect(incident, normal);

        emit_child(
            Ray {
                origin: offset_origin(contact.position, outward_normal, reflected),
                direction: reflected,
            },
            reflectance,
        );

        if !cannot_refract {
            let refracted = refract(incident, normal, eta, cos_theta);

            emit_child(
                Ray {
                    origin: offset_origin(contact.position, outward_normal, refracted),
                    direction: refracted,
                },
                1.0 - reflectance,
            );
        }

        0.0
    }
}
