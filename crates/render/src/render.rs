use antler_camera::{Camera, Observer};
use antler_colour::{Rgb, Rgba};
use antler_image::{RgbaImage, Tile};
use antler_material::Bsdf;
use antler_scene::{Resources, Scene};
use antler_settings::{ImageSettings, RenderSettings};
use antler_shader::Appearance;
use nalgebra::Point2;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use rayon::prelude::*;

use crate::probe::Probe;

pub fn render_probe<R: Rng + SeedableRng>(
    rng: &mut R,
    settings: &RenderSettings,
    resources: &Resources,
    scene: &Scene,
    probe: Probe,
) -> Option<Rgb> {
    if probe.generation >= settings.max_generation || probe.weight <= settings.min_weight {
        return Some(Rgb::BLACK);
    }

    let (object_id, mut contact) = scene.intersection(resources, &probe.ray, f32::INFINITY)?;

    let object = scene.get_object(object_id);
    let shader = resources.get_shader(object.shader_id);
    let material = resources.get_material(object.material_id);

    let mut bounced = Rgb::BLACK;

    let mut child_rng = R::from_rng(rng);
    let local_fraction = material.scatter(rng, &probe.ray, &contact, |child_ray, fraction| {
        let child = probe.child(child_ray, fraction);

        if let Some(colour) = render_probe(&mut child_rng, settings, resources, scene, child) {
            bounced += colour;
        }
    });

    let emitted = shader.emitted(&contact);
    let ambient = scene.ambient_light(rng, resources, object_id, &mut contact) * local_fraction;
    let direct = scene.direct_light(rng, resources, &probe.ray, object_id, &mut contact) * local_fraction;
    let local = emitted + ambient + direct;

    Some(local * probe.weight + bounced)
}

pub fn render_tile<R: Rng + SeedableRng>(
    rng: &mut R,
    image_settings: &ImageSettings,
    render_settings: &RenderSettings,
    camera: &Camera,
    resources: &Resources,
    scene: &Scene,
    tile: Tile,
) -> Vec<Rgba> {
    let image_width = image_settings.resolution[0] as f32;
    let image_height = image_settings.resolution[1] as f32;
    let ss = image_settings.super_samples.max(1);
    let ss_delta = 1.0 / ss as f32;
    let inv_samples = 1.0 / (ss * ss) as f32;

    let [tile_width, tile_height] = tile.size();
    let mut pixels = vec![image_settings.background; tile.num_pixels()];

    for local_y in 0..tile_height {
        let y = tile.min[1] + local_y;

        for local_x in 0..tile_width {
            let x = tile.min[0] + local_x;

            let mut colour = Rgba::TRANSPARENT;

            for sy in 0..ss {
                for sx in 0..ss {
                    let uv = Point2::new(
                        (sx as f32 + 0.5).mul_add(ss_delta, x as f32) / image_width,
                        (sy as f32 + 0.5).mul_add(ss_delta, y as f32) / image_height,
                    );

                    let ray = camera.emit(image_settings.resolution, uv);
                    let probe = Probe::new(ray);

                    let sample = render_probe(rng, render_settings, resources, scene, probe)
                        .map_or(image_settings.background, |rgb| rgb.to_rgba());
                    colour += sample;
                }
            }

            let index = local_y * tile_width + local_x;
            pixels[index] = colour * inv_samples;
        }
    }

    pixels
}

#[must_use]
pub fn render_image(
    image_settings: &ImageSettings,
    render_settings: &RenderSettings,
    camera: &Camera,
    resources: &Resources,
    scene: &Scene,
) -> RgbaImage {
    let tiles = Tile::create_tiles(image_settings.resolution, image_settings.tile_size);

    let rendered_tiles = tiles
        .into_par_iter()
        .map(|tile| {
            let seed = tile_seed(tile.min);
            let mut rng = SmallRng::seed_from_u64(seed);

            (
                tile,
                render_tile(
                    &mut rng,
                    image_settings,
                    render_settings,
                    camera,
                    resources,
                    scene,
                    tile,
                ),
            )
        })
        .collect::<Vec<_>>();

    let mut image = RgbaImage::filled(image_settings.resolution, image_settings.background);
    for (tile, pixels) in rendered_tiles {
        image.apply_tile(tile, &pixels);
    }

    image
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

// #[must_use]
// #[inline]
// fn probe_seed(coord: [usize; 2], sub_coord: [usize; 2]) -> u64 {
//     let [x, y] = coord;
//     let [sx, sy] = sub_coord;

//     let mut z = x as u64;
//     z ^= (y as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
//     z ^= (sx as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);
//     z ^= (sy as u64).wrapping_mul(0x94D0_49BB_1331_11EB);

//     z ^= z >> 30;
//     z = z.wrapping_mul(0xBF58_476D_1CE4_E5B9);
//     z ^= z >> 27;
//     z = z.wrapping_mul(0x94D0_49BB_1331_11EB);
//     z ^ (z >> 31)
// }
