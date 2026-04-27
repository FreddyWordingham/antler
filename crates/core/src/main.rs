use antler::prelude::*;
use nalgebra::{Point2, Point3, Vector3};

fn main() {
    let width = 640;
    let height = 320;
    let resolution = [width, height];

    let shape = Aabb::new(Point3::new(-0.5, -0.5, -0.5), Point3::new(0.5, 0.5, 0.5));

    let camera = Perspective::new(
        Point3::new(4.0, -4.0, 4.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::z_axis(),
        45.0f32.to_radians(),
    );
    let mut image = RgbaImage::filled(resolution, Rgba::TRANSPARENT);

    for y in 0..height {
        for x in 0..width {
            let uv = Point2::new(x as f32 / width as f32, y as f32 / height as f32);
            let ray = camera.emit(resolution, uv);

            if let Some(hit) = shape.hit(&ray) {
                let normal = hit.normal;
                image[(x, y)] = Rgba::new(normal.x.abs(), normal.y.abs(), normal.z.abs(), 1.0);
            }
        }
    }

    image.save("output.png").expect("Failed to save image");
}
