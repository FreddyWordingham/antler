//! Shadowing calculation.

use crate::{
    illumination::visibility,
    input::Shader,
    parts::{Scene, Tracer},
};
use arctk::{
    geom::{Hit, Ray},
    math::{rand_circle_point, rand_hemisphere_point, Dir3},
};
use rand::{rngs::ThreadRng, Rng};
use std::{f64::consts::PI, fmt::Display};

/// Calculate the shadowing factor.
#[inline]
#[must_use]
pub fn shadow<T: Display + Ord>(
    scene: &Scene<T>,
    shader: &Shader,
    ray: &Ray,
    hit: &Hit<&T>,
    rng: &mut ThreadRng,
) -> f64 {
    let bump_dist = scene.sett.bump_dist();

    let sun_dir = Dir3::new_normalize(shader.sky().sun_pos() - ray.pos());
    let mut light_ray = Ray::new(*ray.pos(), *hit.side().norm());
    light_ray.travel(bump_dist);
    *light_ray.dir_mut() = sun_dir;

    let solar = if let Some(samples) = shader.samples().soft_shadows() {
        let offset = rng.gen_range(0.0, 2.0 * PI);
        let mut total = 0.0;
        for n in 0..samples {
            let (r, theta) = rand_circle_point(n, samples);
            let mut soft_ray = light_ray.clone();
            soft_ray.rotate(r * shader.sky().sun_rad(), theta + offset);
            total += visibility(scene, Tracer::new(soft_ray), 1.0);
        }
        total / f64::from(samples)
    } else {
        visibility(scene, Tracer::new(light_ray), 1.0)
    };

    if let Some(samples) = shader.samples().ambient_occlusion() {
        let offset = rng.gen_range(0.0, 2.0 * PI);
        let mut total = 0.0;
        let mut norm_ray = Ray::new(*ray.pos(), *hit.side().norm());
        norm_ray.travel(bump_dist);
        for n in 0..samples {
            let (phi, theta) = rand_hemisphere_point(n, samples);
            let mut ambient_ray = norm_ray.clone();
            ambient_ray.rotate(phi, theta + offset);
            total += visibility(scene, Tracer::new(ambient_ray), 1.0);
        }
        let ambient = (total / f64::from(samples)).powi(shader.shadow().ao_pow());

        return ambient.mul_add(*shader.shadow().ambient(), solar * shader.shadow().direct());
    };

    solar
}
