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

    let Some((object_id, hit)) = scene.trace(&probe.ray) else {
        return Rgb::BLACK;
    };

    let object = scene.get_object(object_id);
    let shader = world.get_shader(object.shader_id);
    let material = world.get_material(object.material_id);

    let scatter = material.scatter(&probe, &hit);

    let emitted = shader.emitted(&hit);
    let reflected = shader.reflected(&probe, &hit) * (probe.weight * scatter.absorbed);
    let local_colour = emitted + reflected;

    let bounced_colours = scatter
        .children
        .into_iter()
        .map(|(fraction, child)| render(world, scene, probe.child(child, fraction)))
        .fold(Rgb::BLACK, |a, b| a + b);

    local_colour + bounced_colours
}
