use crate::errors::ParseHexError;

pub fn parse_hex<const N: usize>(hex: &str, expected: &'static [usize]) -> Result<[u8; N], ParseHexError> {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    let bytes = hex.as_bytes();

    match bytes.len() {
        len if len == N => {
            let mut out = [0u8; N];

            for i in 0..N {
                let nibble = hex_nibble(bytes[i])?;
                out[i] = (nibble << 4) | nibble;
            }

            Ok(out)
        }
        len if len == N * 2 => {
            let mut out = [0u8; N];

            for i in 0..N {
                out[i] = hex_byte(bytes[i * 2], bytes[i * 2 + 1])?;
            }

            Ok(out)
        }
        found => Err(ParseHexError::InvalidLength { expected, found }),
    }
}

fn hex_nibble(byte: u8) -> Result<u8, ParseHexError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => u8::from_str_radix(std::str::from_utf8(&[byte]).unwrap_or("�"), 16)
            .map(|_| unreachable!())
            .map_err(ParseHexError::from),
    }
}

fn hex_byte(high: u8, low: u8) -> Result<u8, ParseHexError> {
    Ok((hex_nibble(high)? << 4) | hex_nibble(low)?)
}
