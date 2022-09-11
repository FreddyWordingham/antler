use arctk::{
    args,
    parse::json,
    phys::Crossing,
    render::{run, Attribute, Input, Output, Parameters},
    rt::{Camera, Ray},
};
use lazy_static::lazy_static;
use nalgebra::{Unit, Vector3};
use palette::{Gradient, LinSrgba};
use rand::{rngs::ThreadRng, Rng};
use std::{
    f64::consts::{FRAC_PI_2, PI},
    path::PathBuf,
    time::Instant,
};

lazy_static! {
    /// Golden ratio constant.
    static ref GOLDEN_RATIO: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
}

fn main() {
    args!(
        _bin_path: PathBuf,
        output_dir: PathBuf,
        parameters_path: PathBuf
    );
    let parameters = json::load::<Parameters>(&parameters_path);
    run(&parameters, &output_dir, sample);
}

/// Sample the scene.
#[inline]
fn sample(
    input: &Input,
    camera: &Camera,
    mut ray: Ray,
    mut weight: f64,
    pixel: [usize; 2],
    data: &mut Output,
    rng: &mut ThreadRng,
) {
    // Watch time.
    let start_time = Instant::now();

    // Unpack.
    let settings = &input.settings;
    let tree = &input.tree;
    let _shader = &input.shader;

    // Common constants.
    let bump_dist = settings.bump_dist;
    let loop_limit = settings.loop_limit;
    let min_weight = settings.min_weight;
    let max_dist = settings.max_distance;

    // Main event loop.
    let mut num_loops = 0;
    while let Some(hit) = tree.scan(ray.clone(), bump_dist, max_dist) {
        // Loop limit check.
        if num_loops >= loop_limit {
            println!("[WARN] : Terminating tracer: loop limit reached.");
            break;
        }
        num_loops += 1;

        // Weight pruning.
        if weight < min_weight {
            break;
        }

        // Handle collision.
        let norm = hit.side.norm();
        match hit.tag {
            Attribute::Opaque(grad) => {
                ray.travel(hit.dist);
                weight = colour(
                    input, camera, &mut ray, weight, pixel, data, rng, norm, grad, 1.0,
                );
                break;
            }
            Attribute::Mirror(grad, abs_frac) => {
                ray.travel(hit.dist);
                weight = colour(
                    input, camera, &mut ray, weight, pixel, data, rng, norm, grad, *abs_frac,
                );
                ray.dir = Crossing::calc_ref_dir(&ray.dir, norm);
                ray.travel(bump_dist);
            }
            Attribute::Transparent(grad, abs_frac) => {
                ray.travel(hit.dist);
                weight = colour(
                    input, camera, &mut ray, weight, pixel, data, rng, norm, grad, *abs_frac,
                );
                ray.travel(bump_dist);
            }
            Attribute::Refractive(grad, abs_frac, [inside, outside]) => {
                ray.travel(hit.dist);
                colour(
                    input, camera, &mut ray, weight, pixel, data, rng, norm, grad, *abs_frac,
                );

                let [curr, next] = if hit.side.is_inside() {
                    [inside, outside]
                } else {
                    [outside, inside]
                };
                let crossing = Crossing::new(&ray.dir, norm, *curr, *next);

                // Transmission ray.
                if let Some(trans_dir) = crossing.trans_dir {
                    let mut trans_ray = ray.clone();
                    trans_ray.dir = trans_dir;
                    trans_ray.travel(bump_dist);

                    sample(
                        input,
                        camera,
                        trans_ray,
                        weight * crossing.trans_prob(),
                        pixel,
                        data,
                        rng,
                    );
                    break;
                }

                // Continuing reflection ray.
                weight *= crossing.ref_prob;
                ray.dir = crossing.ref_dir;
                ray.travel(bump_dist);
            }
            Attribute::Luminous(grad, bright_mult) => {
                ray.travel(hit.dist);
                colour(
                    input,
                    camera,
                    &mut ray,
                    weight,
                    pixel,
                    data,
                    rng,
                    norm,
                    grad,
                    *bright_mult,
                );
                weight = 0.0;
                break;
            }
            Attribute::Switchable([grad_0, grad_1], x) => {
                ray.travel(hit.dist);
                if ray.pos.z < *x {
                    colour(
                        input, camera, &mut ray, weight, pixel, data, rng, norm, grad_0, 1.0,
                    );
                } else {
                    colour(
                        input, camera, &mut ray, weight, pixel, data, rng, norm, grad_1, 1.0,
                    );
                }
                weight = 0.0;
                break;
            }
            _ => {}
        }
    }

    sky_colour(input, &ray, weight, data, pixel);

    // Record time.
    data.time[pixel] += start_time.elapsed().as_micros() as f64;
}

/// Determine the colour of a ray-surface collision.
/// Record the data.
#[allow(clippy::too_many_arguments)]
#[inline]
fn colour(
    input: &Input,
    camera: &Camera,
    ray: &mut Ray,
    weight: f64,
    pixel: [usize; 2],
    data: &mut Output,
    rng: &mut ThreadRng,
    norm: &Unit<Vector3<f64>>,
    grad: &Gradient<LinSrgba>,
    abs_frac: f64,
) -> f64 {
    debug_assert!(abs_frac > 0.0);
    debug_assert!(abs_frac <= 1.0);

    // Colour calculation.
    let shadow = shadow(input, rng, ray, norm);
    let light = light(input, camera, ray, norm);
    let base_col = grad.get(light as f32);
    let col = Gradient::new(vec![LinSrgba::default(), base_col]).get(shadow as f32);

    // Colouring.
    data.colour[pixel] += col * (abs_frac * weight) as f32;

    weight * (1.0 - abs_frac)
}

/// Calculate the lighting factor.
/// Zero indicates darkness.
/// Unity indicates fully illuminated.
#[inline]
#[must_use]
pub fn light(input: &Input, camera: &Camera, ray: &Ray, norm: &Unit<Vector3<f64>>) -> f64 {
    let light_dir = Unit::new_normalize(input.shader.sun_pos - ray.pos);
    let view_dir = Unit::new_normalize(camera.orient.pos - ray.pos);
    let ref_dir = Crossing::calc_ref_dir(&ray.dir, norm);

    let [ambient, mut diffuse, mut specular] = input.shader.light;
    diffuse *= norm.dot(&light_dir);
    specular *= view_dir.dot(&ref_dir).max(0.0).powi(input.shader.spec_pow);

    ambient + diffuse + specular
}

/// Calculate the shadowing factor.
/// Zero completely enshrouded.
/// Unity no shadows.
#[inline]
#[must_use]
pub fn shadow(input: &Input, rng: &mut ThreadRng, ray: &Ray, norm: &Unit<Vector3<f64>>) -> f64 {
    let bump_dist = input.settings.bump_dist;

    let sun_dir = Unit::new_normalize(input.shader.sun_pos - ray.pos);
    let mut light_ray = Ray::new(ray.pos, *norm);
    light_ray.travel(bump_dist);
    light_ray.dir = sun_dir;

    let solar = if let Some((samples, rad)) = input.shader.soft_shadow_samples {
        let offset = rng.gen_range(0.0..(2.0 * PI));
        let mut total = 0.0;
        for n in 0..samples {
            let (r, theta) = rand_circle_point(n, samples);
            let mut soft_ray = light_ray.clone();
            soft_ray.rotate(r * rad, theta + offset);
            total += occlusion(input, soft_ray, input.shader.occ_dist[1]);
        }
        total / f64::from(samples)
    } else {
        occlusion(input, light_ray, input.shader.occ_dist[1])
    };

    if let Some((samples, power)) = input.shader.ambient_shadow_samples {
        let offset = rng.gen_range(0.0..(2.0 * PI));
        let mut total = 0.0;
        let mut norm_ray = Ray::new(ray.pos, *norm);
        norm_ray.travel(bump_dist);
        for n in 0..samples {
            let (phi, theta) = rand_hemisphere_point(n, samples);
            let mut ambient_ray = norm_ray.clone();
            ambient_ray.rotate(phi, theta + offset);
            total += occlusion(input, ambient_ray, input.shader.occ_dist[1]);
        }
        let ambient = (total / f64::from(samples)).powi(power);

        return ambient.mul_add(input.shader.shadow[0], solar * input.shader.shadow[1]);
    };

    solar
}

/// Calculate the occlusion experienced over a distance along ray.
/// Zero indicates full occlusion.
/// Unity indicates full view.
#[inline]
#[must_use]
pub fn occlusion(input: &Input, mut ray: Ray, mut dist: f64) -> f64 {
    debug_assert!(dist > 0.0);

    let bump_dist = input.settings.bump_dist;
    let loop_limit = input.settings.loop_limit;
    let min_weight = input.settings.min_weight;

    let mut vis = 1.0;
    let mut num_loops = 0;
    while let Some(hit) = input.tree.scan(ray.clone(), bump_dist, dist) {
        // Loop limit check.
        if num_loops >= loop_limit {
            println!("[WARN] : Terminating shadower: loop limit reached.");
            return 0.0;
        }
        num_loops += 1;

        // Check if we've flown far enough.
        dist -= hit.dist;
        if dist < 0.0 {
            return vis;
        }

        // Check if it's still worth going.
        if vis < min_weight {
            return 0.0;
        }

        // Handle collision.
        match hit.tag {
            Attribute::Opaque(..) | Attribute::Switchable(..) => {
                return vis / dist.mul_add(input.shader.fall_off, 1.0);
            }
            Attribute::Mirror(.., abs_frac) => {
                ray.travel(dist);
                vis *= 1.0 - abs_frac;
                ray.dir = Crossing::calc_ref_dir(&ray.dir, hit.side.norm());
                ray.travel(bump_dist);
            }
            Attribute::Transparent(.., abs_frac) => {
                ray.travel(dist + bump_dist);
                vis *= 1.0 - abs_frac;
            }
            Attribute::Refractive(.., abs_frac, [_inside, _outside]) => {
                ray.travel(dist + bump_dist);
                vis *= 1.0 - abs_frac;
            }
            Attribute::Luminous(.., bright_mult) => {
                return (vis * bright_mult) / dist.mul_add(input.shader.fall_off, 1.0);
            }
            _ => {}
        }
    }

    vis
}

/// Sample points within a circle using the golden ratio.
#[inline]
#[must_use]
pub fn rand_circle_point(n: i32, max: i32) -> (f64, f64) {
    debug_assert!(n >= 0);
    debug_assert!(n < max);

    let r = f64::from(n) / f64::from(max - 1);
    let theta = f64::from(n) * *GOLDEN_RATIO;

    (r, theta)
}

/// Sample points on a sphere's surface using the golden ratio.
#[inline]
#[must_use]
pub fn rand_sphere_point(n: i32, max: i32) -> (f64, f64) {
    debug_assert!(n >= 0);
    debug_assert!(n < max);

    let d = f64::from(1 - max).mul_add(0.5, f64::from(n));
    let phi = ((2.0 * d) / f64::from(max)).asin() + FRAC_PI_2;
    let theta = ((2.0 * PI) / *GOLDEN_RATIO) * (d % *GOLDEN_RATIO);

    (phi, theta)
}

/// Sample points on a hemisphere's surface using the golden ratio.
#[inline]
#[must_use]
pub fn rand_hemisphere_point(n: i32, max: i32) -> (f64, f64) {
    debug_assert!(n >= 0);
    debug_assert!(n < max);

    rand_sphere_point(n, max * 2)
}

/// Determine the colour of the sky.
/// Record the data.
#[inline]
fn sky_colour(input: &Input, ray: &Ray, weight: f64, data: &mut Output, pixel: [usize; 2]) {
    let u = ray.dir.z.abs();
    let col = input.shader.sky_grad.get(u as f32);

    data.colour[pixel] += col * weight as f32;
}
