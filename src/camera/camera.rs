use nalgebra::Point2;

use crate::tracing::Photon;

pub trait Camera {
    fn emit(&self, uv: Point2<f32>) -> Photon;
}
