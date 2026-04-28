use antler_geometry::Ray;
use nalgebra::Point2;

pub trait Observer {
    fn emit(&self, resolution: [usize; 2], uv: Point2<f32>) -> Ray;
}
