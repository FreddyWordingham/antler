use std::{
    fs::File,
    io::{BufReader, BufWriter, Error as IoError, Result as IoResult},
    ops::{Index, IndexMut},
    path::Path,
};

use antler_colour::{Pixel, Rgb, Rgba};
use antler_grid::SurfaceGrid;
use nalgebra::Point2;
use png::{Decoder, Encoder};

use crate::{errors::ImageLoadError, tile::Tile};

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

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ImageLoadError> {
        let file = File::open(path)?;
        let decoder = Decoder::new(BufReader::new(file));
        let mut reader = decoder.read_info()?;

        let output_buffer_size = reader.output_buffer_size().ok_or(ImageLoadError::UnknownBufferSize)?;

        let mut buffer = vec![0; output_buffer_size];
        let info = reader.next_frame(&mut buffer).map_err(IoError::other)?;

        if info.color_type != P::PNG_COLOUR_TYPE {
            return Err(ImageLoadError::ColourTypeMismatch {
                expected: P::PNG_COLOUR_TYPE,
                found: info.color_type,
            });
        }

        let bytes = &buffer[..info.buffer_size()];

        if bytes.len() % P::CHANNELS != 0 {
            return Err(ImageLoadError::InvalidByteLength {
                len: bytes.len(),
                channels: P::CHANNELS,
            });
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

    #[must_use]
    #[inline]
    pub fn sample_nearest(&self, uv: Point2<f32>) -> P {
        let [width, height] = self.pixels.size();

        let u = uv.x.rem_euclid(1.0);
        let v = uv.y.rem_euclid(1.0);

        let x = (u * width as f32).floor() as usize;
        let y = ((1.0 - v) * height as f32).floor() as usize;

        self[(x.min(width - 1), y.min(height - 1))]
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
