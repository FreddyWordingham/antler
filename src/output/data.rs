//! Render output.

use arctk::{
    data::Histogram,
    err::Error,
    file::Save,
    img::Image,
    math::{Dir3, Vec3},
    ord::{X, Y},
};
use ndarray::Array2;
use ndarray_stats::QuantileExt;
use palette::{Gradient, LinSrgba};
use std::{ops::AddAssign, path::Path};

/// Rendering output.
pub struct Data {
    /// Colour image.
    pub img: Image,
    /// Time image.
    pub time: Array2<f64>,
    /// Distance image.
    pub dist: Array2<f64>,
    /// Ending direction.
    pub end_dir: Array2<Vec3>,
}

impl Data {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            img: Image::new_blank(res, LinSrgba::new(0.0, 0.0, 0.0, 0.0)),
            time: Array2::zeros(res),
            dist: Array2::zeros(res),
            end_dir: Array2::default(res),
        }
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.img += &rhs.img;
        self.time += &rhs.time;
        self.dist += &rhs.dist;
        self.end_dir += &rhs.end_dir;
    }
}

impl Save for Data {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        self.img.save(&out_dir.join("render.png"))?;

        let greyscale = Gradient::new(vec![
            LinSrgba::new(0.0, 0.0, 0.0, 1.0),
            LinSrgba::new(1.0, 1.0, 1.0, 1.0),
        ]);
        let redscale = Gradient::new(vec![
            LinSrgba::new(0.0, 0.0, 0.0, 1.0),
            LinSrgba::new(1.0, 0.0, 0.0, 1.0),
        ]);
        let greenscale = Gradient::new(vec![
            LinSrgba::new(0.0, 0.0, 0.0, 1.0),
            LinSrgba::new(0.0, 1.0, 0.0, 1.0),
        ]);
        let bluescale = Gradient::new(vec![
            LinSrgba::new(0.0, 0.0, 0.0, 1.0),
            LinSrgba::new(0.0, 0.0, 1.0, 1.0),
        ]);

        let max_time = self.time.max()?;
        Image::new(self.time.map(|x| greyscale.get((*x / max_time) as f32)))
            .save(&out_dir.join("time.png"))?;
        let mut dist_hist = Histogram::new(0.0, *max_time, 1000);
        for t in &self.time {
            dist_hist.collect(*t);
        }
        dist_hist.save(&out_dir.join("time.csv"))?;

        let max_dist = self.dist.max()?;
        Image::new(self.dist.map(|x| greyscale.get((*x / max_dist) as f32)))
            .save(&out_dir.join("dist.png"))?;
        let mut dist_hist = Histogram::new(0.0, *max_dist, 1000);
        for d in &self.dist {
            dist_hist.collect(*d);
        }
        dist_hist.save(&out_dir.join("dist.csv"))?;

        let max_dir_len = *self.end_dir.map(Vec3::magnitude).max()?;
        Image::new(self.end_dir.map(|v| {
            let dir = Dir3::new_normalize(*v);

            redscale.get((dir.x.abs() / max_dir_len) as f32)
                + greenscale.get((dir.y.abs() / max_dir_len) as f32)
                + bluescale.get((dir.z.abs() / max_dir_len) as f32)
        }))
        .save(&out_dir.join("norm.png"))
    }
}
