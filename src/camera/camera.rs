use nalgebra::Point2;

use crate::{
    camera::{Orthographic, Perspective},
    tracing::Probe,
};

pub trait Camera {
    fn emit(&self, uv: Point2<f32>) -> Probe;
}

pub enum CameraEnum {
    Perspective(Perspective),
    Orthographic(Orthographic),
}

impl Camera for CameraEnum {
    fn emit(&self, uv: Point2<f32>) -> Probe {
        match self {
            CameraEnum::Perspective(perspective) => perspective.emit(uv),
            CameraEnum::Orthographic(orthographic) => orthographic.emit(uv),
        }
    }
}
