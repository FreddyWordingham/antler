//! Rendering order input structure.

use arctk_attr::input;
use rand::{prelude::SliceRandom, thread_rng};

/// Rendering order enumeration.
#[input]
pub enum Order {
    /// Zero to end.
    Forward,
    /// Last to zero.
    Backward,
    /// Random order.
    Shuffle,
}

impl Order {
    /// Generate the order list.
    #[inline]
    #[must_use]
    pub fn list(&self, n: usize) -> Vec<usize> {
        let mut order = (0..n).collect::<Vec<usize>>();
        match *self {
            Self::Forward => {}
            Self::Backward => {
                order.reverse();
            }
            Self::Shuffle => {
                order.as_mut_slice().shuffle(&mut thread_rng());
            }
        };
        order
    }
}
