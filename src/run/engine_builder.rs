//! Engine selection.

use crate::{engines, Engine};
use arctk::{err::Error, file::Build};
use arctk_attr::input;
use std::path::Path;

/// Engine selection.
#[input]
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
