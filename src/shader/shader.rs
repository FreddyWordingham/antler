use crate::{
    colour::Colour,
    tracing::{Hit, Photon},
};

pub trait Shader {
    fn shade(&self, photon: &Photon, hit: &Hit) -> Colour;
}
