mod material;
mod mirror;
mod opaque;
mod reflective;
mod refractive;
mod scatter;

pub use material::{Material, MaterialEnum};
pub use mirror::Mirror;
pub use opaque::Opaque;
pub use reflective::Reflective;
pub use refractive::Refractive;
pub use scatter::Scatter;
