use std::{
    fs::File,
    io::{BufWriter, Error as IoError, Result as IoResult},
    ops::{Index, IndexMut},
    path::Path,
};

use antler_colour::{Pixel, Rgb, Rgba};
use antler_grid::SurfaceGrid;
use png::Encoder;

pub struct Image<P: Pixel> {
    pixels: SurfaceGrid<P>,
}

pub type RgbImage = Image<Rgb>;
pub type RgbaImage = Image<Rgba>;

impl<P: Pixel> Image<P> {
    pub fn filled(size: [usize; 2], pixel: P) -> Self
    where
        P: Clone,
    {
        Self {
            pixels: SurfaceGrid::from_elem(size, pixel),
        }
    }

    pub fn from_vec(size: [usize; 2], data: Vec<P>) -> Self {
        Self {
            pixels: SurfaceGrid::from_vec(size, data),
        }
    }

    #[inline]
    pub const fn width(&self) -> usize {
        self.pixels.size()[0]
    }

    #[inline]
    pub const fn height(&self) -> usize {
        self.pixels.size()[1]
    }

    pub fn save(&self, path: impl AsRef<Path>) -> IoResult<()> {
        let width = u32::try_from(self.width()).expect("image width exceeds u32::MAX");
        let height = u32::try_from(self.height()).expect("image height exceeds u32::MAX");

        let mut bytes = Vec::with_capacity(self.pixels.len() * P::CHANNELS);

        for pixel in self.pixels.as_slice() {
            bytes.extend_from_slice(pixel.to_bytes().as_ref());
        }

        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        let mut encoder = Encoder::new(writer, width, height);
        encoder.set_color(P::PNG_COLOUR_TYPE);
        encoder.set_depth(P::PNG_BIT_DEPTH);

        let mut writer = encoder.write_header().map_err(IoError::other)?;
        writer.write_image_data(&bytes).map_err(IoError::other)
    }
}

impl<P: Pixel> Index<(usize, usize)> for Image<P> {
    type Output = P;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[[x, y]]
    }
}

impl<P: Pixel> IndexMut<(usize, usize)> for Image<P> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[[x, y]]
    }
}
