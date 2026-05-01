use std::fmt::Write;

use png::{BitDepth, ColorType};

use crate::errors::ParseHexError;

pub trait Pixel: Copy {
    const CHANNELS: usize;
    const PNG_COLOUR_TYPE: ColorType;
    const PNG_BIT_DEPTH: BitDepth = BitDepth::Eight;

    type Bytes: AsRef<[u8]>;

    #[must_use]
    fn to_bytes(&self) -> Self::Bytes;

    #[must_use]
    fn from_bytes(bytes: &[u8]) -> Self;

    #[must_use]
    fn to_u32(&self) -> u32;

    #[must_use]
    fn from_u32(value: u32) -> Self;

    #[must_use]
    #[inline]
    fn to_hex(&self) -> String {
        let bytes = self.to_bytes();
        let mut out = String::from("#");
        for byte in bytes.as_ref() {
            let _ = write!(out, "{byte:02X}");
        }
        out
    }

    fn from_hex(hex: &str) -> Result<Self, ParseHexError>
    where
        Self: Sized;
}
