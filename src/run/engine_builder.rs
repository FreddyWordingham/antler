//! Engine selection.

use crate::{
    err::Error,
    file::Build,
    sim::render::{engines, Engine},
};
use arctk_attr::load;
use std::path::Path;

/// Engine selection.
#[load]
#[derive(Clone)]
pub enum EngineBuilder {
    /// Antler rendering engine.
    Antler,
}

impl Build for EngineBuilder {
    type Inst = Engine;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Engine, Error> {
        match self {
            Self::Antler => Ok(engines::antler),
        }
    }
}
