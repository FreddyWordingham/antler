//! Render sampling information structure.

use palette::LinSrgba;

/// Scene sampling return information.
pub struct Sample {
    /// Base colour.
    pub col: LinSrgba,
}

impl Sample {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(col: LinSrgba) -> Self {
        Self { col }
    }
}
