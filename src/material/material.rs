use crate::{
    material::Scatter,
    tracing::{Hit, Photon},
};

pub trait Material {
    fn scatter(&self, photon: &Photon, hit: &Hit) -> Scatter;
}
