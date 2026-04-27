use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseHexError {
    InvalidLength { expected: [usize; 2], found: usize },
    InvalidDigit { byte: u8 },
}

impl Display for ParseHexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::InvalidLength { expected, found } => {
                write!(f, "Invalid hex length {found}, expected one of {expected:?}")
            }
            Self::InvalidDigit { byte } => {
                write!(f, "Invalid hex digit: {:?}", char::from(*byte))
            }
        }
    }
}

impl Error for ParseHexError {}
