use std::{
    fs::File,
    io::{BufWriter, Error as IoError, Result as IoResult},
    ops::{Index, IndexMut},
    path::Path,
};

use png::Encoder;

use crate::{
    colour::{Colour, Rgb, Rgba},
    storage::Grid,
};

pub struct Image<C: Colour> {
    pixels: Grid<C>,
}

pub type RgbImage = Image<Rgb>;
pub type RgbaImage = Image<Rgba>;

impl<C: Colour> Image<C> {
    pub fn filled(size: [usize; 2], colour: C) -> Self
    where
        C: Clone,
    {
        Self {
            pixels: Grid::from_elem(size, colour),
        }
    }

    pub fn from_vec(size: [usize; 2], data: Vec<C>) -> Self {
        assert_eq!(
            data.len(),
            size[0] * size[1],
            "Data length ({}) must match size area ({})",
            data.len(),
            size[0] * size[1]
        );
        Self {
            pixels: Grid::from_vec(size, data),
        }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.pixels.len()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }

    #[inline]
    pub const fn size(&self) -> [usize; 2] {
        self.pixels.size()
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

        let mut bytes = Vec::with_capacity(self.width() * self.height() * C::CHANNELS);

        for y in 0..self.height() {
            for x in 0..self.width() {
                bytes.extend_from_slice(self[(x, y)].to_bytes().as_ref());
            }
        }

        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        let mut encoder = Encoder::new(writer, width, height);
        encoder.set_color(C::PNG_COLOUR_TYPE);
        encoder.set_depth(C::PNG_BIT_DEPTH);

        let mut writer = encoder.write_header().map_err(IoError::other)?;
        writer.write_image_data(&bytes).map_err(IoError::other)
    }
}

impl<C: Colour> Index<(usize, usize)> for Image<C> {
    type Output = C;

    fn index(&self, coord: (usize, usize)) -> &Self::Output {
        &self.pixels[coord]
    }
}

impl<C: Colour> IndexMut<(usize, usize)> for Image<C> {
    fn index_mut(&mut self, coord: (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[coord]
    }
}
