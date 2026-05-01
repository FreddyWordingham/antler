use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

use png::{BitDepth, ColorType, DecodingError};

#[derive(Debug)]
pub enum ImageLoadError {
    Io(IoError),
    Png(DecodingError),
    UnknownBufferSize,
    ColourTypeMismatch { expected: ColorType, found: ColorType },
    BitDepthMismatch { expected: BitDepth, found: BitDepth },
    InvalidByteLength { len: usize, channels: usize },
}

impl From<IoError> for ImageLoadError {
    fn from(err: IoError) -> Self {
        Self::Io(err)
    }
}

impl From<DecodingError> for ImageLoadError {
    fn from(err: DecodingError) -> Self {
        Self::Png(err)
    }
}

impl Display for ImageLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Io(err) => write!(f, "IO error: {err}"),
            Self::Png(err) => write!(f, "PNG decoding error: {err}"),
            Self::UnknownBufferSize => write!(f, "PNG output buffer size is unknown"),
            Self::ColourTypeMismatch { expected, found } => {
                write!(f, "PNG colour type mismatch: expected {:?}, got {:?}", expected, found)
            }
            Self::BitDepthMismatch { expected, found } => {
                write!(f, "PNG bit depth mismatch: expected {:?}, got {:?}", expected, found)
            }
            Self::InvalidByteLength { len, channels } => write!(
                f,
                "Invalid byte length for pixel data: expected a multiple of {}, got {}",
                channels, len
            ),
        }
    }
}

impl Error for ImageLoadError {}
