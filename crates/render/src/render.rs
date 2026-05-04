use std::time::{Duration, Instant};

use antler_camera::{Camera, Observer};
use antler_colour::{Rgb, Rgba};
use antler_image::{RgbaImage, Tile};
use antler_material::Bsdf;
use antler_scene::{Resources, Scene};
use antler_settings::{ImageSettings, LightingSettings, ProbeSettings, ResolveMode};
use nalgebra::Point2;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use rayon::prelude::*;

use crate::{probe::Probe, utils::progress_bar};

pub fn render_probe<R: Rng + SeedableRng>(
    rng: &mut R,
    lighting_settings: &LightingSettings,
    probe_settings: &ProbeSettings,
    resources: &Resources,
    scene: &Scene,
    probe: Probe,
    skybox_is_visible_to_camera: bool,
) -> Option<Rgb> {
    if probe.generation >= probe_settings.max_generation || probe.weight <= probe_settings.min_weight {
        return Some(Rgb::BLACK);
    }

    let Some((object_id, mut contact)) = scene.intersection(resources, &probe.ray, f32::INFINITY) else {
        if probe.generation == 0 && !skybox_is_visible_to_camera {
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

        if let Some(colour) = render_probe(
            &mut child_rng,
            lighting_settings,
            probe_settings,
            resources,
            scene,
            child,
            skybox_is_visible_to_camera,
        ) {
            bounced += colour;
        }
    });

    let ao = scene.occlusion(rng, resources, &mut contact);

    let emitted = object.emissive.as_ref().map_or(Rgb::BLACK, |e| e.colour * e.intensity);
    let ambient = scene.ambient_shade(shader, &probe.ray, &contact) * ao * local_fraction;
    let direct = scene.direct_light(rng, resources, &probe.ray, object_id, &mut contact) * local_fraction;

    let local =
        emitted * lighting_settings.emitted + ambient * lighting_settings.ambient + direct * lighting_settings.direct;

    Some(local * probe.weight + bounced * lighting_settings.indirect)
}

pub fn render_tile<R: Rng + SeedableRng>(
    rng: &mut R,
    image_settings: &ImageSettings,
    lighting_settings: &LightingSettings,
    probe_settings: &ProbeSettings,
    camera: &Camera,
    resources: &Resources,
    scene: &Scene,
    tile: Tile,
) -> (Vec<Rgba>, Vec<Duration>) {
    let image_width = image_settings.resolution[0] as f32;
    let image_height = image_settings.resolution[1] as f32;
    let ss = image_settings.super_samples.max(1);
    let ss_delta = 1.0 / ss as f32;

    let [tile_width, tile_height] = tile.size();
    let mut pixels = vec![image_settings.background; tile.num_pixels()];
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

                    let ray = camera.emit(image_settings.resolution, uv);
                    let probe = Probe::new(ray);

                    let start_time = Instant::now();

                    let sample = render_probe(
                        rng,
                        lighting_settings,
                        probe_settings,
                        resources,
                        scene,
                        probe,
                        image_settings.art_config.skybox_is_visible_to_camera,
                    )
                    .map_or(image_settings.background, |c: Rgb| Rgb::to_rgba(&c));

                    total_duration += start_time.elapsed();

                    samples.push(sample);
                }
            }

            let index = local_y * tile_width + local_x;
            pixels[index] = resolve_samples([x, y], &samples, &image_settings.art_config.resolve_mode);
            times.push(total_duration);
        }
    }

    (pixels, times)
}

#[must_use]
pub fn render_image(
    image_settings: &ImageSettings,
    lighting_settings: &LightingSettings,
    probe_settings: &ProbeSettings,
    camera: &Camera,
    resources: &Resources,
    scene: &Scene,
) -> (RgbaImage, RgbaImage) {
    let tiles = Tile::create_tiles(image_settings.resolution, image_settings.tile_size);

    let pb = progress_bar(tiles.len() as u64);
    let rendered_tiles = tiles
        .into_par_iter()
        .map(|tile| {
            pb.inc(1);

            let seed = tile_seed(tile.min);
            let mut rng = SmallRng::seed_from_u64(seed);

            (
                tile,
                render_tile(
                    &mut rng,
                    image_settings,
                    lighting_settings,
                    probe_settings,
                    camera,
                    resources,
                    scene,
                    tile,
                ),
            )
        })
        .collect::<Vec<_>>();
    pb.finish();

    let (min_time, max_time) = rendered_tiles
        .iter()
        .flat_map(|(_, (_, times))| times.iter())
        .fold((Duration::MAX, Duration::ZERO), |(min, max), &t| {
            (min.min(t), max.max(t))
        });

    let mut visual_image = RgbaImage::filled(image_settings.resolution, image_settings.background);
    let mut temporal_image = RgbaImage::filled(image_settings.resolution, Rgba::TRANSPARENT);

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

#[must_use]
fn resolve_samples(pixel: [usize; 2], samples: &[Rgba], resolve_mode: &ResolveMode) -> Rgba {
    match resolve_mode {
        ResolveMode::Average => average_samples(samples),
        ResolveMode::PaletteAware { palette, dither } => resolve_palette_aware(pixel, samples, palette, *dither),
    }
}

#[must_use]
fn average_samples(samples: &[Rgba]) -> Rgba {
    if samples.is_empty() {
        return Rgba::TRANSPARENT;
    }

    let inv_samples = 1.0 / samples.len() as f32;

    samples
        .iter()
        .copied()
        .fold(Rgba::TRANSPARENT, |acc, sample| acc + sample)
        * inv_samples
}

#[must_use]
fn resolve_palette_aware(pixel: [usize; 2], samples: &[Rgba], palette: &[Rgb], dither: bool) -> Rgba {
    if samples.is_empty() {
        return Rgba::TRANSPARENT;
    }

    if palette.is_empty() {
        return average_samples(samples);
    }

    let mut weights = vec![0.0_f32; palette.len()];
    let mut alpha = 0.0;

    for sample in samples {
        alpha += sample.alpha;

        if sample.alpha <= 0.0 {
            continue;
        }

        let rgb = sample.to_rgb();
        let (index, _) = nearest_palette_colour(rgb, palette);

        // Vote by coverage/opacity, not inverse colour distance.
        weights[index] += sample.alpha;
    }

    let rgb = if dither {
        choose_with_bayer_dither(pixel, palette, &weights)
    } else {
        choose_highest_weight(palette, &weights)
    };

    let mut rgba = rgb.to_rgba();
    rgba.alpha = alpha / samples.len() as f32;
    rgba
}

#[must_use]
fn nearest_palette_colour(colour: Rgb, palette: &[Rgb]) -> (usize, f32) {
    palette
        .iter()
        .copied()
        .enumerate()
        .map(|(index, candidate)| {
            let dr = colour.red - candidate.red;
            let dg = colour.green - candidate.green;
            let db = colour.blue - candidate.blue;

            let distance = dr * dr + dg * dg + db * db;

            (index, distance)
        })
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .expect("palette must not be empty")
}

#[must_use]
fn choose_highest_weight(palette: &[Rgb], weights: &[f32]) -> Rgb {
    let index = weights
        .iter()
        .copied()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map_or(0, |(index, _)| index);

    palette[index]
}

const BAYER_4X4: [[f32; 4]; 4] = [
    [0.0, 8.0, 2.0, 10.0],
    [12.0, 4.0, 14.0, 6.0],
    [3.0, 11.0, 1.0, 9.0],
    [15.0, 7.0, 13.0, 5.0],
];

#[must_use]
fn choose_with_bayer_dither(pixel: [usize; 2], palette: &[Rgb], weights: &[f32]) -> Rgb {
    let mut ranked = weights.iter().copied().enumerate().collect::<Vec<_>>();

    ranked.sort_by(|(_, a), (_, b)| b.total_cmp(a));

    let Some(&(a, wa)) = ranked.first() else {
        return palette[0];
    };

    let Some(&(b, wb)) = ranked.get(1) else {
        return palette[a];
    };

    let total = wa + wb;

    if total <= 0.0 {
        return palette[a];
    }

    let probability_b = wb / total;
    let threshold = (BAYER_4X4[pixel[1] % 4][pixel[0] % 4] + 0.5) / 16.0;

    if threshold < probability_b {
        palette[b]
    } else {
        palette[a]
    }
}
