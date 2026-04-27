mod material;
mod mirror;
mod opaque;
mod reflective;
mod refractive;

pub use material::{Material, MaterialEnum};
pub use mirror::Mirror;
pub use opaque::Opaque;
pub use reflective::Reflective;
pub use refractive::Refractive;
