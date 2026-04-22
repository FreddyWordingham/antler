use antler::prelude::*;
use nalgebra::{Point3, Similarity3, Unit, Vector3};

fn main() {
    let width = 400;
    let height = 300;

    let mut world = World::new();

    let sphere_id = world.add_geometry(GeometryEnum::Sphere(Sphere {
        centre: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
    }));

    let shader_id = world.add_shader(ShaderEnum::Block(Block {
        colour: Rgb::new(0.2, 0.8, 0.4),
    }));

    let material_id = world.add_material(MaterialEnum::Opaque(Opaque));

    let mut scene = Scene::new();
    scene.add_object(Object::new(sphere_id, shader_id, material_id, Similarity3::identity()));

    let mut image = RgbImage::filled([width, height], Rgb::BLACK);

    let camera_origin = Point3::new(0.0, 0.0, -3.0);

    for y in 0..height {
        for x in 0..width {
            let u = (x as f32 + 0.5) / width as f32;
            let v = (y as f32 + 0.5) / height as f32;

            let sx = 2.0 * u - 1.0;
            let sy = 1.0 - 2.0 * v;

            let direction = Unit::new_normalize(Vector3::new(sx, sy, 1.5));

            let ray = Ray {
                origin: camera_origin,
                direction,
            };

            let probe = Probe::new(WorldRay::new(ray));
            let colour = render(&world, &scene, probe);
            image[(x, y)] = colour;
        }
    }

    image.save("output.png").unwrap();
}
