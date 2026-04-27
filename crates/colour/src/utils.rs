use crate::errors::ParseHexError;

pub fn parse_hex<const N: usize>(hex: &str) -> Result<[u8; N], ParseHexError> {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    let bytes = hex.as_bytes();

    match bytes.len() {
        len if len == N => parse_short_hex(bytes),
        len if len == N * 2 => parse_long_hex(bytes),
        found => Err(ParseHexError::InvalidLength {
            expected: [N, N * 2],
            found,
        }),
    }
}

fn parse_short_hex<const N: usize>(bytes: &[u8]) -> Result<[u8; N], ParseHexError> {
    let mut out = [0u8; N];

    for i in 0..N {
        let n = hex_nibble(bytes[i])?;
        out[i] = (n << 4) | n;
    }

    Ok(out)
}

fn parse_long_hex<const N: usize>(bytes: &[u8]) -> Result<[u8; N], ParseHexError> {
    let mut out = [0u8; N];

    for i in 0..N {
        out[i] = hex_byte(bytes[i * 2], bytes[i * 2 + 1])?;
    }

    Ok(out)
}

fn hex_nibble(byte: u8) -> Result<u8, ParseHexError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(ParseHexError::InvalidDigit { byte }),
    }
}

fn hex_byte(high: u8, low: u8) -> Result<u8, ParseHexError> {
    Ok((hex_nibble(high)? << 4) | hex_nibble(low)?)
}
