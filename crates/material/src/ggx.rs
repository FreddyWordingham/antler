use std::f32::consts::PI;

use antler_geometry::{
    Contact, Ray,
    utils::{offset_origin, tangent_frame},
};
use nalgebra::{Unit, Vector3};
use rand::{Rng, RngExt};

use crate::{bsdf::Bsdf, utils::reflect};

pub struct Ggx {
    roughness: f32,
    reflectance: f32,
}

impl Ggx {
    #[must_use]
    pub const fn new(roughness: f32, reflectance: f32) -> Self {
        Self {
            roughness: roughness.clamp(0.001, 1.0),
            reflectance: reflectance.clamp(0.0, 1.0),
        }
    }
}

impl Bsdf for Ggx {
    fn scatter<R: Rng, F: FnMut(Ray, f32)>(&self, rng: &mut R, ray: &Ray, contact: &Contact, mut emit_child: F) -> f32 {
        let half_vector = sample_ggx_half_vector(rng, contact.normal, self.roughness);
        let reflected = reflect(ray.direction, half_vector);

        if reflected.dot(&contact.normal) > 0.0 {
            emit_child(
                Ray {
                    origin: offset_origin(contact.position, contact.normal, reflected),
                    direction: reflected,
                },
                self.reflectance,
            );
        }

        0.0
    }
}

#[inline]
fn sample_ggx_half_vector<R: Rng>(rng: &mut R, normal: Unit<Vector3<f32>>, roughness: f32) -> Unit<Vector3<f32>> {
    let u1: f32 = rng.random();
    let u2: f32 = rng.random();

    let alpha = roughness * roughness;

    let phi = 2.0 * PI * u1;
    let cos_theta = ((1.0 - u2) / alpha.mul_add(alpha, -1.0).mul_add(u2, 1.0)).sqrt();
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let local = Vector3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta);

    let (tangent, bitangent) = tangent_frame(normal);

    Unit::new_normalize(*tangent * local.x + *bitangent * local.y + *normal * local.z)
}
