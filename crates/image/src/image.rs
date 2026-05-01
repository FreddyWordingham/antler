use std::{
    fs::File,
    io::{BufReader, BufWriter, Error as IoError, Result as IoResult},
    ops::{Index, IndexMut},
    path::Path,
};

use antler_colour::{Pixel, Rgb, Rgba};
use antler_grid::SurfaceGrid;
use png::{Decoder, Encoder};

use crate::tile::Tile;

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

    #[must_use]
    pub fn from_vec(size: [usize; 2], data: Vec<P>) -> Self {
        Self {
            pixels: SurfaceGrid::from_vec(size, data),
        }
    }

    #[inline]
    pub fn apply_tile(&mut self, tile: Tile, pixels: &[P]) {
        let [tile_width, tile_height] = tile.size();

        for local_y in 0..tile_height {
            let y = tile.min[1] + local_y;

            for local_x in 0..tile_width {
                let x = tile.min[0] + local_x;
                self[(x, y)] = pixels[local_y * tile_width + local_x];
            }
        }
    }

    pub fn load(path: impl AsRef<Path>) -> IoResult<Self> {
        let file = File::open(path)?;
        let decoder = Decoder::new(BufReader::new(file));
        let mut reader = decoder.read_info().map_err(IoError::other)?;

        let output_buffer_size = reader
            .output_buffer_size()
            .ok_or_else(|| IoError::other("PNG output buffer size is unknown"))?;

        let mut buffer = vec![0; output_buffer_size];
        let info = reader.next_frame(&mut buffer).map_err(IoError::other)?;

        if info.color_type != P::PNG_COLOUR_TYPE {
            return Err(IoError::other(format!(
                "PNG colour type mismatch: expected {:?}, got {:?}",
                P::PNG_COLOUR_TYPE,
                info.color_type,
            )));
        }

        if info.bit_depth != P::PNG_BIT_DEPTH {
            return Err(IoError::other(format!(
                "PNG bit depth mismatch: expected {:?}, got {:?}",
                P::PNG_BIT_DEPTH,
                info.bit_depth,
            )));
        }

        let bytes = &buffer[..info.buffer_size()];

        if bytes.len() % P::CHANNELS != 0 {
            return Err(IoError::other(
                "PNG byte length is not divisible by pixel channel count",
            ));
        }

        let pixels = bytes.chunks_exact(P::CHANNELS).map(P::from_bytes).collect();

        Ok(Self::from_vec(
            [
                usize::try_from(info.width).expect("image width exceeds usize::MAX"),
                usize::try_from(info.height).expect("image height exceeds usize::MAX"),
            ],
            pixels,
        ))
    }

    pub fn save(&self, path: impl AsRef<Path>) -> IoResult<()> {
        let width = u32::try_from(self.pixels.size()[0]).expect("image width exceeds u32::MAX");
        let height = u32::try_from(self.pixels.size()[1]).expect("image height exceeds u32::MAX");

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

    fn index(&self, coord: (usize, usize)) -> &Self::Output {
        &self.pixels[coord.into()]
    }
}

impl<P: Pixel> IndexMut<(usize, usize)> for Image<P> {
    fn index_mut(&mut self, coord: (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[coord.into()]
    }
}
