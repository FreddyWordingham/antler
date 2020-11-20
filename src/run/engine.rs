//! Engine function alias.

use crate::{Input, Output, Tracer};
use rand::rngs::ThreadRng;

/// Rendering sampling engine function type.
pub type Engine =
    fn(input: &Input, &mut ThreadRng, trace: Tracer, data: &mut Output, pixel: [usize; 2]);
