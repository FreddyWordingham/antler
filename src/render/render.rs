use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::ParallelSliceMut,
};

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

    let mut image = RgbImage::filled(resolution, Rgb::BLACK);

    image
        .pixels_mut()
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, pixel) in row.iter_mut().enumerate() {
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

                *pixel = colour * inv_samples;
            }
        });

    image
}
