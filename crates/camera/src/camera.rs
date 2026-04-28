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

impl From<Orthographic> for Camera {
    #[inline]
    fn from(val: Orthographic) -> Self {
        Self::Orthographic(val)
    }
}

impl From<Perspective> for Camera {
    #[inline]
    fn from(val: Perspective) -> Self {
        Self::Perspective(val)
    }
}
