use png::{BitDepth, ColorType};

use crate::errors::ParseHexError;

pub trait Pixel: Copy {
    const CHANNELS: usize;
    const PNG_COLOUR_TYPE: ColorType;
    const PNG_BIT_DEPTH: BitDepth = BitDepth::Eight;

    type Bytes: AsRef<[u8]>;

    fn to_bytes(&self) -> Self::Bytes;
    fn from_bytes(bytes: Self::Bytes) -> Self;

    fn to_u32(&self) -> u32;
    fn from_u32(value: u32) -> Self;

    #[inline]
    fn to_hex(&self) -> String {
        let bytes = self.to_bytes();
        let mut out = String::from("#");
        for byte in bytes.as_ref() {
            out.push_str(&format!("{byte:02X}"));
        }
        out
    }

    fn from_hex(hex: &str) -> Result<Self, ParseHexError>
    where
        Self: Sized;
}
