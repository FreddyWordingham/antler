use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseHexError {
    InvalidLength { expected: &'static [usize], found: usize },
    InvalidDigit(ParseIntError),
}

impl From<ParseIntError> for ParseHexError {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidDigit(value)
    }
}

impl Display for ParseHexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::InvalidLength { expected, found } => {
                write!(f, "Invalid hex length {found}, expected one of {expected:?}")
            }
            Self::InvalidDigit(err) => {
                write!(f, "Invalid hex digit: {err}")
            }
        }
    }
}

impl Error for ParseHexError {}
