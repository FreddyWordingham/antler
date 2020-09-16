//! Render output.

use arctk::{
    img::Image,
    ord::{X, Y},
};
use palette::LinSrgba;
use std::ops::AddAssign;

/// Rendering output.
pub struct Data {
    /// Colour image.
    pub img: Image,
}

impl Data {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            img: Image::new(res, LinSrgba::new(0.0, 0.0, 0.0, 0.0)),
        }
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.img += &rhs.img;
    }
}
