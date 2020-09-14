//! Lighting samples setup structure.

use arctk::clone;
use arctk_attr::input;

/// Lighting structure.
#[input]
pub struct Samples {
    /// Optional number of ambient occlusion samples.
    ambient_occlusion: Option<i32>,
    /// Optional number of soft shadow samples.
    soft_shadows: Option<i32>,
}

impl Samples {
    clone!(ambient_occlusion, Option<i32>);
    clone!(soft_shadows, Option<i32>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ambient_occlusion: Option<i32>, soft_shadows: Option<i32>) -> Self {
        debug_assert!(ambient_occlusion.is_none() || ambient_occlusion.unwrap() > 1);
        debug_assert!(soft_shadows.is_none() || soft_shadows.unwrap() > 1);

        Self {
            ambient_occlusion,
            soft_shadows,
        }
    }
}
