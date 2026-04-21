use crate::tracing::Photon;

pub struct Scatter {
    pub local_weight: f32,
    pub children: Vec<Photon>,
}
