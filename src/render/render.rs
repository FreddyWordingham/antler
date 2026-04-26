use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::{
    camera::Camera,
    colour::{Rgb, Rgba},
    material::Material,
    render::RenderSettings,
    shader::Shader,
    storage::RgbaImage,
    tracing::Probe,
    utils::seed,
    world::{Scene, World},
};

const MAX_GENERATION: u32 = 5;
const MIN_WEIGHT: f32 = 1.0e-2;
const TILE_SIZE: usize = 32;

#[derive(Debug, Clone, Copy)]
struct Tile {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

pub fn render_probe(world: &World, scene: &Scene, probe: Probe) -> Option<Rgb> {
    render_probe_rgb(world, scene, probe)
}

fn render_probe_rgb(world: &World, scene: &Scene, mut probe: Probe) -> Option<Rgb> {
    if probe.generation >= MAX_GENERATION || probe.weight <= MIN_WEIGHT {
        return Some(Rgb::BLACK);
    }

    let world_hit = scene.trace(world, &probe.ray)?;

    let object = scene.get_object(world_hit.object_id);
    let shader = world.get_shader(object.shader_id);
    let material = world.get_material(object.material_id);

    let scatter = material.scatter(&probe, &world_hit);

    let emitted = shader.emitted(&world_hit);
    let ambient = scene.ambient_light(world, &world_hit, probe.rng()) * scatter.local_fraction;
    let direct = scene.direct_light(world, &probe.ray.clone(), &world_hit, probe.rng()) * scatter.local_fraction;

    let mut bounced = Rgb::BLACK;

    for (fraction, child_ray) in scatter.children {
        let child_probe = probe.child(child_ray, fraction);

        if let Some(colour) = render_probe_rgb(world, scene, child_probe) {
            bounced += colour;
        }
    }

    Some(emitted + ambient + direct + bounced)
}

pub fn render_image<C>(world: &World, scene: &Scene, camera: &C, settings: RenderSettings) -> RgbaImage
where
    C: Camera + Sync,
{
    render_image_inner(world, scene, camera, settings, None)
}

pub fn render_image_with_progress<C>(
    world: &World,
    scene: &Scene,
    camera: &C,
    settings: RenderSettings,
    label: impl Into<String>,
    done_label: impl Into<String>,
) -> RgbaImage
where
    C: Camera + Sync,
{
    let tile_count = tile_count(settings.resolution);

    let progress = ProgressBar::new(tile_count as u64);
    progress.set_message(label.into());
    progress.set_style(
        ProgressStyle::with_template("{msg} [{wide_bar:.cyan/blue}] {pos}/{len} tiles ({elapsed})")
            .unwrap()
            .progress_chars("=>-"),
    );
    let image = render_image_inner(world, scene, camera, settings, Some(&progress));

    progress.finish_with_message(done_label.into());

    image
}

fn render_image_inner<C>(
    world: &World,
    scene: &Scene,
    camera: &C,
    settings: RenderSettings,
    progress: Option<&ProgressBar>,
) -> RgbaImage
where
    C: Camera + Sync,
{
    let width = settings.resolution[0];
    let height = settings.resolution[1];
    let ss = settings.super_samples.max(1);
    let ss_delta = 1.0 / ss as f32;
    let inv_samples = 1.0 / (ss * ss) as f32;

    let tiles = make_tiles(settings.resolution);

    let rendered_tiles: Vec<(Tile, Vec<Rgba>)> = tiles
        .into_par_iter()
        .map(|tile| {
            let tile_width = tile.x1 - tile.x0;
            let tile_height = tile.y1 - tile.y0;
            let mut pixels = vec![settings.background; tile_width * tile_height];

            for local_y in 0..tile_height {
                let y = tile.y0 + local_y;

                for local_x in 0..tile_width {
                    let x = tile.x0 + local_x;
                    let mut colour = Rgba::TRANSPARENT;

                    for sy in 0..ss {
                        for sx in 0..ss {
                            let uv = nalgebra::Point2::new(
                                (x as f32 + (sx as f32 + 0.5) * ss_delta) / width as f32,
                                (y as f32 + (sy as f32 + 0.5) * ss_delta) / height as f32,
                            );

                            let probe_seed = seed::pixel_seed([x, y], [sx as usize, sy as usize]);
                            let world_ray = camera.emit(uv, settings.resolution);
                            let probe = Probe::with_seed(world_ray, probe_seed);

                            let sample = match render_probe(world, scene, probe) {
                                Some(rgb) => rgb.to_rgba(),
                                None => settings.background,
                            };
                            colour += sample;
                        }
                    }

                    pixels[local_y * tile_width + local_x] = colour * inv_samples;
                }
            }

            if let Some(progress) = progress {
                progress.inc(1);
            }

            (tile, pixels)
        })
        .collect();

    let mut image = RgbaImage::filled(settings.resolution, settings.background);

    for (tile, pixels) in rendered_tiles {
        let tile_width = tile.x1 - tile.x0;
        let tile_height = tile.y1 - tile.y0;

        for local_y in 0..tile_height {
            let y = tile.y0 + local_y;

            for local_x in 0..tile_width {
                let x = tile.x0 + local_x;
                image[(x, y)] = pixels[local_y * tile_width + local_x];
            }
        }
    }

    image
}

fn make_tiles(resolution: [usize; 2]) -> Vec<Tile> {
    let width = resolution[0];
    let height = resolution[1];

    let tiles_x = width.div_ceil(TILE_SIZE);
    let tiles_y = height.div_ceil(TILE_SIZE);

    (0..tiles_y)
        .flat_map(|ty| {
            (0..tiles_x).map(move |tx| {
                let x0 = tx * TILE_SIZE;
                let y0 = ty * TILE_SIZE;
                let x1 = (x0 + TILE_SIZE).min(width);
                let y1 = (y0 + TILE_SIZE).min(height);

                Tile { x0, y0, x1, y1 }
            })
        })
        .collect()
}

fn tile_count(resolution: [usize; 2]) -> usize {
    let tiles_x = resolution[0].div_ceil(TILE_SIZE);
    let tiles_y = resolution[1].div_ceil(TILE_SIZE);

    tiles_x * tiles_y
}
