use antler::prelude::*;
use nalgebra::{Point2, Point3, Vector2, Vector3};

fn main() {
    let width = 320;
    let height = 320;
    let resolution = [width, height];

    // let shape = Aabb::new(Point3::new(-0.5, -0.5, -0.5), Point3::new(0.5, 0.5, 0.5));
    // let shape = Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.5);
    // let shape = Mesh::load("assets/meshes/golem.obj").expect("Failed to load mesh");
    // let shape = Torus::new(Point3::new(0.0, 0.0, 0.0), 0.5, 0.25);
    // let shape = Capsule::new(Point3::new(-0.5, 0.0, 0.0), Point3::new(0.5, 0.0, 0.0), 0.05);
    // let shape = Circle::new(Point3::new(0.0, 0.0, 0.0), Vector3::y_axis(), 0.5);
    let shape = Quad::new(Point3::new(0.0, 0.0, 0.0), Vector3::y_axis(), Vector2::new(1.0, 0.4));

    let camera = Perspective::new(
        Point3::new(4.0, -4.0, 4.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::z_axis(),
        15.0f32.to_radians(),
    );
    let mut image = RgbaImage::filled(resolution, Rgba::TRANSPARENT);

    for y in 0..height {
        for x in 0..width {
            let uv = Point2::new(x as f32 / width as f32, y as f32 / height as f32);
            let ray = camera.emit(resolution, uv);

            if let Some(intersect) = shape.intersection(&ray, f32::INFINITY) {
                let normal = intersect.normal;
                image[(x, y)] = Rgba::new(normal.x.abs(), normal.y.abs(), normal.z.abs(), 1.0);
            }
        }
    }

    image.save("output.png").expect("Failed to save image");
}
