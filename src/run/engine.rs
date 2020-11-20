//! Engine function alias.

use crate::sim::render::{Input, Output, Tracer};
use rand::rngs::ThreadRng;

/// MCRT sampling engine function type.
pub type Engine =
    fn(input: &Input, &mut ThreadRng, trace: Tracer, data: &mut Output, pixel: [usize; 2]);
