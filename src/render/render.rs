use crate::{
    colour::Rgb,
    tracing::Photon,
    world::{Scene, World},
};

const MAX_GENERATION: u32 = 5;
const MIN_WEIGHT: f32 = 0.01;

pub fn render(world: &World, scene: &Scene, photon: Photon) -> Rgb {
    if photon.generation >= MAX_GENERATION || photon.weight <= MIN_WEIGHT {
        return Rgb::BLACK;
    }

    let Some((object_id, hit)) = scene.trace(&photon.ray) else {
        return Rgb::BLACK;
    };

    let object = scene.get_object(object_id);
    let shader = world.get_shader(object.shader_id);
    let material = world.get_material(object.material_id);

    let scatter = material.scatter(&photon, &hit);
    let local_colour = shader.shade(&photon, &hit) * scatter.local_weight;

    let bounced_colours = scatter
        .children
        .into_iter()
        .map(|child| render(world, scene, child))
        .fold(Rgb::BLACK, |a, b| a + b);

    local_colour + bounced_colours
}
