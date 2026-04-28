use antler::prelude::*;
use nalgebra::{Point3, Similarity3, Unit, Vector2, Vector3};

fn main() {
    // Resources
    let mut resources = Resources::new();

    // Geometry
    let aabb_id = resources.add_geometry(Aabb::new(Point3::new(-0.5, -0.5, -0.5), Point3::new(0.5, 0.5, 0.5)));
    let sphere_id = resources.add_geometry(Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.5));
    let mesh_id = resources.add_geometry(Mesh::load("assets/meshes/golem.obj").expect("Failed to load mesh"));
    let torus_id = resources.add_geometry(Torus::new(Point3::new(0.0, 0.0, 0.0), 0.5, 0.25));
    let capsule_id = resources.add_geometry(Capsule::new(
        Point3::new(-0.5, 0.0, 0.0),
        Point3::new(0.5, 0.0, 0.0),
        0.05,
    ));
    let circle_id = resources.add_geometry(Circle::new(Point3::new(0.0, 0.0, 0.0), Vector3::y_axis(), 0.5));
    let quad_id = resources.add_geometry(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::y_axis(),
        Vector2::new(1.0, 0.4),
    ));

    // Material
    let ggx_id = resources.add_material(Ggx::new(0.8, 0.2));
    let lambertian_id = resources.add_material(Lambertian::new(0.8));
    let mirror_id = resources.add_material(Mirror::new());
    let opaque_id = resources.add_material(Opaque::new());

    // Shader
    let block_id = resources.add_shader(Block::new(Rgb::new(0.8, 0.2, 0.2)));
    let checkerboard_id =
        resources.add_shader(Checkerboard::new(0.2, Rgb::new(0.8, 0.8, 0.8), Rgb::new(0.2, 0.2, 0.2)));
    let luminous_id = resources.add_shader(Luminous::new(Rgb::new(0.8, 0.8, 0.2), 2.0));
    let solid_id = resources.add_shader(Solid::new(Rgb::new(0.2, 0.8, 0.2)));

    // Objects
    let centrepiece = Object::new(sphere_id, opaque_id, solid_id, Similarity3::identity());

    // Scene
    let mut scene = Scene::new();
    scene.add_light(
        Directional::new(
            Unit::new_normalize(Vector3::new(-1.0, 1.0, -1.0)),
            Rgb::WHITE,
            Some(3.0_f32.to_radians()),
            Some(8),
        )
        .into(),
    );
    scene.add_object(centrepiece);

    // Camera
    let camera = Perspective::new(
        Point3::new(4.0, -4.0, 4.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::z_axis(),
        30.0f32.to_radians(),
    );

    // Image
    let image_settings = ImageSettings {
        background: Rgba::TRANSPARENT,
        resolution: [320, 320],
        tile_size: [16, 16],
        super_samples: 2,
    };

    // Render
    let render_settings = RenderSettings {
        max_generation: 5,
        min_weight: 1.0e-2,
    };

    // Run
    let image = render_image(&image_settings, &render_settings, &camera.into(), &resources, &scene);
    image.save("output.png").expect("Failed to save image");
}
