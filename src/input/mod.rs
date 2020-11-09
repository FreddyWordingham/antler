//! Input.

pub mod settings;
pub mod shader;
pub mod shader_builder;

#[cfg(feature = "window")]
pub mod scale_builder;

#[cfg(feature = "window")]
pub use self::scale_builder::*;

pub use self::{settings::*, shader::*, shader_builder::*};
