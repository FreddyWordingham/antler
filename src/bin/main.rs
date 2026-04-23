use antler::prelude::*;
use indicatif::ProgressBar;
use nalgebra::{Point2, Point3, Similarity3, Unit, Vector3};

fn main() {
    // Create output directory if it doesn't exist
    std::fs::create_dir_all("output").unwrap();

    let width = 800;
    let height = 600;

    let mut world = World::new();

    let _sphere_id = world.add_geometry(GeometryEnum::Sphere(Sphere {
        centre: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
    }));
    let aabb_id = world.add_geometry(GeometryEnum::Aabb(Aabb {
        min: Point3::new(-1.0, -1.0, -1.0),
        max: Point3::new(1.0, 1.0, 1.0),
    }));
    let quad_id = world.add_geometry(GeometryEnum::Quad(Quad {
        position: Point3::new(0.0, 0.0, 0.0),
        normal: Vector3::z_axis(),
        size: [10.0, 4.0],
    }));

    let green_lambertion_shader_id = world.add_shader(ShaderEnum::Lambertion(Lambertion {
        colour: Rgb::new(0.2, 0.8, 0.4),
    }));
    let grey_lambertion_shader_id = world.add_shader(ShaderEnum::Lambertion(Lambertion {
        colour: Rgb::new(0.5, 0.5, 0.5),
    }));

    let material_id = world.add_material(MaterialEnum::Opaque(Opaque));

    let mut scene = Scene::new();
    scene.add_light(LightEnum::DirectionalLight(DirectionalLight {
        direction: Unit::new_normalize(Vector3::new(-5.0, -2.0, -2.0)),
        radiance: Rgb::new(1.0, 1.0, 1.0),
    }));
    scene.add_object(Object::new(
        aabb_id,
        grey_lambertion_shader_id,
        material_id,
        Similarity3::identity(),
    ));
    scene.add_object(Object::new(
        quad_id,
        green_lambertion_shader_id,
        material_id,
        Similarity3::new(Vector3::new(0.0, 0.0, -1.0), Vector3::zeros(), 1.0),
    ));
    scene.build(&world);

    let mut image = RgbImage::filled([width, height], Rgb::BLACK);

    let camera = create_perspective_camera([width, height]);
    // let camera = create_orthographic_camera([width, height]);

    let pb = create_progress_bar(width * height);
    for y in 0..height {
        for x in 0..width {
            pb.inc(1);

            let uv = Point2::new((x as f32 + 0.5) / width as f32, (y as f32 + 0.5) / height as f32);
            let probe = camera.emit(uv);
            image[(x, y)] = render(&world, &scene, probe);
        }
    }
    pb.finish();

    image.save("output/output.png").unwrap();
}

#[allow(dead_code)]
fn create_perspective_camera(resolution: [usize; 2]) -> Perspective {
    Perspective::new(
        Point3::new(10.0, 10.0, 10.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::z_axis(),
        30.0_f32.to_radians(),
        resolution[0] as f32 / resolution[1] as f32,
    )
}

#[allow(dead_code)]
fn create_orthographic_camera(resolution: [usize; 2]) -> Orthographic {
    let width = 14.0;
    let height = width * resolution[1] as f32 / resolution[0] as f32;

    Orthographic::new(
        Point3::new(10.0, 10.0, 10.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::z_axis(),
        width,
        height,
    )
}

fn create_progress_bar(total: usize) -> ProgressBar {
    let progress_bar = ProgressBar::new(total as u64);
    progress_bar.set_style(
        indicatif::ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("##-"),
    );
    progress_bar
}
