use antler::prelude::*;
use nalgebra::{Point3, Similarity3, Unit, Vector3};

fn main() {
    std::fs::create_dir_all("output").unwrap();

    let width = 1200;
    let height = 900;
    let super_samples = 4;

    let mut world = World::new();

    let quad_id = world.add_geometry(GeometryEnum::Quad(Quad {
        position: Point3::new(0.0, 0.0, 0.0),
        normal: Vector3::z_axis(),
        size: [100.0, 100.0],
    }));

    let cube_id = world.add_geometry(GeometryEnum::Aabb(Aabb {
        min: Point3::new(-1.0, -1.0, -1.0),
        max: Point3::new(1.0, 1.0, 1.0),
    }));

    let green_lambertion_shader_id = world.add_shader(ShaderEnum::Lambertion(Lambertion {
        colour: Rgb::new(0.2, 0.8, 0.4),
    }));

    let checkerboard_shader_id = world.add_shader(ShaderEnum::Checkerboard(Checkerboard {
        size: 0.26,
        colour_a: Rgb::new(1.0, 0.0, 1.0),
        colour_b: Rgb::new(0.2, 0.2, 0.2),
    }));

    let material_id = world.add_material(MaterialEnum::Opaque(Opaque));

    let mut scene = Scene::new();
    scene.add_light(LightEnum::DirectionalLight(DirectionalLight {
        direction: Unit::new_normalize(Vector3::new(-5.0, -2.0, -2.0)),
        radiance: Rgb::new(1.0, 1.0, 1.0),
    }));

    scene.add_object(Object::new(
        quad_id,
        green_lambertion_shader_id,
        material_id,
        Similarity3::identity(),
    ));

    scene.add_object(Object::new(
        cube_id,
        checkerboard_shader_id,
        material_id,
        Similarity3::new(Vector3::new(0.0, 0.0, 2.0), Vector3::zeros(), 2.0),
    ));

    scene.build(&world);

    let camera = create_perspective_camera([width, height]);
    let image = render_image(&world, &scene, &camera, [width, height], super_samples);

    image.save("output/output.png").unwrap();
}

fn create_perspective_camera(resolution: [usize; 2]) -> Perspective {
    Perspective::new(
        Point3::new(10.0, 10.0, 10.0),
        Point3::new(0.0, 0.0, 3.0),
        Vector3::z_axis(),
        45.0_f32.to_radians(),
        resolution[0] as f32 / resolution[1] as f32,
    )
}
