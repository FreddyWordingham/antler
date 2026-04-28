use antler_geometry::Ray;
use nalgebra::Point2;

use crate::{observer::Observer, orthographic::Orthographic, perspective::Perspective};

pub enum Camera {
    Orthographic(Orthographic),
    Perspective(Perspective),
}

impl Observer for Camera {
    fn emit(&self, resolution: [usize; 2], uv: Point2<f32>) -> Ray {
        match self {
            Self::Orthographic(camera) => camera.emit(resolution, uv),
            Self::Perspective(camera) => camera.emit(resolution, uv),
        }
    }
}
