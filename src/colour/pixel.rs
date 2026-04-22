use png::{BitDepth, ColorType};

pub trait Pixel {
    const CHANNELS: usize;
    const PNG_COLOUR_TYPE: ColorType;
    const PNG_BIT_DEPTH: BitDepth = BitDepth::Eight;

    type Bytes: AsRef<[u8]>;

    fn to_bytes(&self) -> Self::Bytes;
    fn from_bytes(bytes: Self::Bytes) -> Self;

    fn to_hex(&self) -> String {
        let bytes = self.to_bytes();
        match Self::CHANNELS {
            3 => format!(
                "#{:02X}{:02X}{:02X}",
                bytes.as_ref()[0],
                bytes.as_ref()[1],
                bytes.as_ref()[2]
            ),
            4 => format!(
                "#{:02X}{:02X}{:02X}{:02X}",
                bytes.as_ref()[0],
                bytes.as_ref()[1],
                bytes.as_ref()[2],
                bytes.as_ref()[3]
            ),
            _ => unreachable!(),
        }
    }
    fn from_hex(hex: &str) -> Self;
}
