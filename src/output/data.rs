//! Render output.

use arctk::{
    err::Error,
    file::Save,
    img::Image,
    ord::{X, Y},
};
use ndarray::Array2;
use palette::{Gradient, LinSrgba};
use std::{ops::AddAssign, path::Path};

/// Rendering output.
pub struct Data {
    /// Colour image.
    pub img: Image,
    /// Time data.
    pub time: Array2<f64>,
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
            time: Array2::zeros(res),
        }
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.img += &rhs.img;
        self.time += &rhs.time;
    }
}

impl Save for Data {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        self.img.save(&out_dir.join("render.png"))?;

        let grad = Gradient::new(vec![
            LinSrgba::new(0.0, 0.0, 0.0, 1.0),
            LinSrgba::new(1.0, 1.0, 1.0, 1.0),
        ]);
        Image::data_to_linear(&(&self.time + 1.0e-12), &grad)?.save(&out_dir.join("time.png"))?;
        Image::data_to_log(&(&self.time + 1.0), &grad)?.save(&out_dir.join("log_time.png"))
    }
}
