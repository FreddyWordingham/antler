use crate::{
    colour::Rgb,
    material::Material,
    shader::Shader,
    tracing::Probe,
    world::{Scene, World},
};

const MAX_GENERATION: u32 = 5;
const MIN_WEIGHT: f32 = 0.01;

pub fn render(world: &World, scene: &Scene, probe: Probe) -> Rgb {
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
    let reflected = shader.reflected(&probe, &world_hit) * (probe.weight * scatter.absorbed);
    let local_colour = emitted + reflected;

    let bounced_colours = scatter
        .children
        .into_iter()
        .map(|(fraction, child)| render(world, scene, probe.child(child, fraction)))
        .fold(Rgb::BLACK, |a, b| a + b);

    local_colour + bounced_colours
}
