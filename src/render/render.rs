use rayon::prelude::*;

use crate::{
    camera::Camera,
    colour::Rgb,
    material::Material,
    shader::Shader,
    storage::RgbImage,
    tracing::Probe,
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

pub fn render_probe(world: &World, scene: &Scene, probe: Probe) -> Rgb {
    if probe.generation >= MAX_GENERATION || probe.weight <= MIN_WEIGHT {
        return Rgb::BLACK;
    }

    let Some(world_hit) = scene.trace(world, &probe.ray) else {
        return Rgb::BLACK;
    };

    let object = scene.get_object(world_hit.object_id);
    let shader = world.get_shader(object.shader_id);
    let material = world.get_material(object.material_id);

    let scatter = material.scatter(&probe, &world_hit);

    let emitted = shader.emitted(&world_hit);
    let direct = scene.direct_light(world, &probe.ray, &world_hit) * scatter.local_fraction;
    let local_colour = emitted + direct;

    let bounced_colours = scatter
        .children
        .into_iter()
        .map(|(fraction, child)| render_probe(world, scene, probe.child(child, fraction)))
        .fold(Rgb::BLACK, |a, b| a + b);

    local_colour + bounced_colours
}

pub fn render_image<C>(
    world: &World,
    scene: &Scene,
    camera: &C,
    resolution: [usize; 2],
    super_samples: usize,
) -> RgbImage
where
    C: Camera + Sync,
{
    let width = resolution[0];
    let height = resolution[1];
    let ss = super_samples.max(1);
    let ss_delta = 1.0 / ss as f32;
    let inv_samples = 1.0 / (ss * ss) as f32;

    let tiles_x = width.div_ceil(TILE_SIZE);
    let tiles_y = height.div_ceil(TILE_SIZE);

    let tiles: Vec<Tile> = (0..tiles_y)
        .flat_map(|ty| {
            (0..tiles_x).map(move |tx| {
                let x0 = tx * TILE_SIZE;
                let y0 = ty * TILE_SIZE;
                let x1 = (x0 + TILE_SIZE).min(width);
                let y1 = (y0 + TILE_SIZE).min(height);

                Tile { x0, y0, x1, y1 }
            })
        })
        .collect();

    let rendered_tiles: Vec<(Tile, Vec<Rgb>)> = tiles
        .into_par_iter()
        .map(|tile| {
            let tile_width = tile.x1 - tile.x0;
            let tile_height = tile.y1 - tile.y0;
            let mut pixels = vec![Rgb::BLACK; tile_width * tile_height];

            for local_y in 0..tile_height {
                let y = tile.y0 + local_y;

                for local_x in 0..tile_width {
                    let x = tile.x0 + local_x;
                    let mut colour = Rgb::BLACK;

                    for sy in 0..ss {
                        for sx in 0..ss {
                            let uv = nalgebra::Point2::new(
                                (x as f32 + (sx as f32 + 0.5) * ss_delta) / width as f32,
                                (y as f32 + (sy as f32 + 0.5) * ss_delta) / height as f32,
                            );

                            let probe = camera.emit(uv);
                            colour += render_probe(world, scene, probe);
                        }
                    }

                    pixels[local_y * tile_width + local_x] = colour * inv_samples;
                }
            }

            (tile, pixels)
        })
        .collect();

    let mut image = RgbImage::filled(resolution, Rgb::BLACK);

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
