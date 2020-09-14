//! Illumination calculation.

pub mod light;
pub mod shadow;
pub mod visibility;

pub use self::{light::*, shadow::*, visibility::*};
