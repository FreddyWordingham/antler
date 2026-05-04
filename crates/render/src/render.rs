use std::time::{Duration, Instant};

use antler_camera::{Camera, Observer};
use antler_colour::{Rgb, Rgba};
use antler_image::{RgbaImage, Tile};
use antler_material::Bsdf;
use antler_parameters::RenderParameters;
use antler_scene::{Resources, Scene};
use nalgebra::Point2;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use rayon::prelude::*;

use crate::{probe::Probe, utils::progress_bar};

pub fn render_probe<R: Rng + SeedableRng>(
    rng: &mut R,
    params: &RenderParameters,
    resources: &Resources,
    scene: &Scene,
    probe: Probe,
) -> Option<Rgb> {
    if probe.generation >= params.probe.max_generation || probe.weight <= params.probe.min_weight {
        return Some(Rgb::BLACK);
    }

    let Some((object_id, mut contact)) = scene.intersection(resources, &probe.ray, f32::INFINITY) else {
        if probe.generation == 0 && !params.image.skybox_is_visible {
            return None;
        }
        return Some(scene.environment_radiance(probe.ray.direction) * probe.weight);
    };

    let object = scene.get_object(object_id);
    let shader = resources.get_shader(object.shader_id);
    let material = resources.get_material(object.material_id);

    let mut bounced = Rgb::BLACK;

    let mut child_rng = R::from_rng(rng);
    let local_fraction = material.scatter(rng, &probe.ray, &contact, |child_ray, fraction| {
        let child = probe.child(child_ray, fraction);

        if let Some(colour) = render_probe(&mut child_rng, params, resources, scene, child) {
            bounced += colour;
        }
    });

    let ao = scene.occlusion(rng, resources, &mut contact);

    let emitted = object.emissive.as_ref().map_or(Rgb::BLACK, |e| e.colour * e.intensity);
    let ambient = scene.ambient_shade(shader, &probe.ray, &contact) * ao * local_fraction;
    let direct = scene.direct_light(rng, resources, &probe.ray, object_id, &mut contact) * local_fraction;

    let local = emitted * params.lighting.emitted + ambient * params.lighting.ambient + direct * params.lighting.direct;

    Some(local * probe.weight + bounced * params.lighting.indirect)
}

pub fn render_tile<R: Rng + SeedableRng>(
    rng: &mut R,
    params: &RenderParameters,
    camera: &Camera,
    resources: &Resources,
    scene: &Scene,
    tile: Tile,
) -> (Vec<Rgba>, Vec<Duration>) {
    let image_width = params.image.resolution[0] as f32;
    let image_height = params.image.resolution[1] as f32;
    let ss = params.image.super_samples.max(1);
    let ss_delta = 1.0 / ss as f32;

    let [tile_width, tile_height] = tile.size();
    let mut pixels = vec![params.image.background; tile.num_pixels()];
    let mut times = Vec::with_capacity(tile.num_pixels());

    for local_y in 0..tile_height {
        let y = tile.min[1] + local_y;

        for local_x in 0..tile_width {
            let x = tile.min[0] + local_x;

            let mut total_duration = Duration::ZERO;
            let mut samples = Vec::with_capacity(ss * ss);

            for sy in 0..ss {
                for sx in 0..ss {
                    let uv = Point2::new(
                        (sx as f32 + 0.5).mul_add(ss_delta, x as f32) / image_width,
                        (sy as f32 + 0.5).mul_add(ss_delta, y as f32) / image_height,
                    );

                    let ray = camera.emit(params.image.resolution, uv);
                    let probe = Probe::new(ray);

                    let start_time = Instant::now();

                    let sample = render_probe(rng, params, resources, scene, probe)
                        .map_or(params.image.background, |c| c.to_rgba());

                    total_duration += start_time.elapsed();

                    samples.push(sample);
                }
            }

            let index = local_y * tile_width + local_x;
            pixels[index] = params.style.resolve([x, y], &samples);
            times.push(total_duration);
        }
    }

    (pixels, times)
}

#[must_use]
pub fn render_image(
    params: &RenderParameters,
    camera: &Camera,
    resources: &Resources,
    scene: &Scene,
) -> (RgbaImage, RgbaImage) {
    let tiles = Tile::create_tiles(params.image.resolution, params.image.tile_size);

    let pb = progress_bar(tiles.len() as u64);
    let rendered_tiles = tiles
        .into_par_iter()
        .map(|tile| {
            pb.inc(1);

            let seed = tile_seed(tile.min);
            let mut rng = SmallRng::seed_from_u64(seed);

            (tile, render_tile(&mut rng, params, camera, resources, scene, tile))
        })
        .collect::<Vec<_>>();
    pb.finish();

    let (min_time, max_time) = rendered_tiles
        .iter()
        .flat_map(|(_, (_, times))| times.iter())
        .fold((Duration::MAX, Duration::ZERO), |(min, max), &t| {
            (min.min(t), max.max(t))
        });

    let mut visual_image = RgbaImage::filled(params.image.resolution, params.image.background);
    let mut temporal_image = RgbaImage::filled(params.image.resolution, Rgba::TRANSPARENT);

    for (tile, (pixels, times)) in rendered_tiles {
        visual_image.apply_tile(tile, &pixels);
        temporal_image.apply_tile(
            tile,
            &times
                .iter()
                .map(|time| {
                    let t = normalise_log_duration(*time, min_time, max_time);
                    Rgba::new(t, t, t, 1.0)
                })
                .collect::<Vec<_>>(),
        );
    }

    (visual_image, temporal_image)
}

#[must_use]
#[inline]
fn normalise_log_duration(time: Duration, min: Duration, max: Duration) -> f32 {
    let min = min.as_secs_f32().max(f32::EPSILON).ln();
    let max = max.as_secs_f32().max(f32::EPSILON).ln();
    let value = time.as_secs_f32().max(f32::EPSILON).ln();

    ((value - min) / (max - min).max(f32::EPSILON)).clamp(0.0, 1.0)
}

#[must_use]
#[inline]
const fn tile_seed(coord: [usize; 2]) -> u64 {
    let [x, y] = coord;

    let mut z = (x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (y as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);

    z ^= z >> 30;
    z = z.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z ^= z >> 27;
    z = z.wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
