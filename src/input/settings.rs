//! Rendering simulation structure.

use arctk::clone;
use arctk_attr::input;

/// Loadable render settings structure.
#[input]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
    /// Maximum visibility tracing [m].
    visibility_dist: f64,
    /// Number of pixels to simulate in each thread block.
    block_size: u64,
    /// Minimum photon weight.
    min_weight: f64,
}

impl Settings {
    clone!(bump_dist, f64);
    clone!(visibility_dist, f64);
    clone!(block_size, u64);
    clone!(min_weight, f64);
}
