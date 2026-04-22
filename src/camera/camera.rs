use nalgebra::Point2;

use crate::tracing::Probe;

pub trait Camera {
    fn emit(&self, uv: Point2<f32>) -> Probe;
}

pub enum CameraEnum {}
