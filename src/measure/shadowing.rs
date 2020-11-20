//! Shadowing calculation.

use crate::{
    geom::Ray,
    math::{rand_circle_point, rand_hemisphere_point, Dir3},
    sim::render::{occlusion, Input},
};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Calculate the shadowing factor.
/// Zero completely enshrouded.
/// Unity no shadows.
#[inline]
#[must_use]
pub fn shadowing(input: &Input, rng: &mut ThreadRng, ray: &Ray, norm: &Dir3) -> f64 {
    let bump_dist = input.sett.bump_dist();

    let sun_dir = Dir3::new_normalize(input.shader.sun_pos() - ray.pos());
    let mut light_ray = Ray::new(*ray.pos(), *norm);
    light_ray.travel(bump_dist);
    *light_ray.dir_mut() = sun_dir;

    let solar = if let Some((samples, rad)) = input.shader.soft_shadow_samples() {
        let offset = rng.gen_range(0.0, 2.0 * PI);
        let mut total = 0.0;
        for n in 0..samples {
            let (r, theta) = rand_circle_point(n, samples);
            let mut soft_ray = light_ray.clone();
            soft_ray.rotate(r * rad, theta + offset);
            total += occlusion(input, soft_ray, input.shader.occ_dist()[1]);
        }
        total / f64::from(samples)
    } else {
        occlusion(input, light_ray, input.shader.occ_dist()[1])
    };

    if let Some((samples, power)) = input.shader.ambient_shadow_samples() {
        let offset = rng.gen_range(0.0, 2.0 * PI);
        let mut total = 0.0;
        let mut norm_ray = Ray::new(*ray.pos(), *norm);
        norm_ray.travel(bump_dist);
        for n in 0..samples {
            let (phi, theta) = rand_hemisphere_point(n, samples);
            let mut ambient_ray = norm_ray.clone();
            ambient_ray.rotate(phi, theta + offset);
            total += occlusion(input, ambient_ray, input.shader.occ_dist()[1]);
        }
        let ambient = (total / f64::from(samples)).powi(power);

        return ambient.mul_add(input.shader.shadow()[0], solar * input.shader.shadow()[1]);
    };

    solar
}
