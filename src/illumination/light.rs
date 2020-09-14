//! Lighting calculation.

use crate::input::Shader;
use arctk::{
    geom::{Hit, Ray},
    math::Dir3,
    phys::Crossing,
};

/// Calculate the lighting factor.
#[inline]
#[must_use]
pub fn light<T>(shader: &Shader, ray: &Ray, hit: &Hit<T>) -> f64 {
    let light_dir = Dir3::new_normalize(shader.sky().sun_pos() - ray.pos());
    let view_dir = Dir3::new_normalize(shader.cam().focus().orient().pos() - ray.pos());
    let ref_dir = Crossing::calc_ref_dir(ray.dir(), hit.side().norm());

    let mut ambient = 1.0;
    let mut diffuse = hit.side().norm().dot(&light_dir).max(0.0);
    let mut specular = view_dir
        .dot(&ref_dir)
        .max(0.0)
        .powi(shader.light().spec_pow());

    ambient *= shader.light().ambient();
    diffuse *= shader.light().diffuse();
    specular *= shader.light().specular();

    ambient + diffuse + specular
}
