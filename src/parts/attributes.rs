//! Surface attributes structure.

use arctk_attr::input;
use std::fmt::{Display, Formatter, Result};

/// Surface rendering attributes.
#[input]
pub enum Attributes {
    /// Luminous.
    Luminous,
    /// Transparent.
    Transparent {
        /// Absorption fraction.
        abs: f64,
    },
    /// Mirror.
    Mirror {
        /// Absorption fraction.
        abs: f64,
    },
    /// Refractive.
    Refractive {
        /// Absorption fraction.
        abs: f64,
        /// Inside refractive index.
        inside: f64,
        /// Outside refractive index.
        outside: f64,
    },
}

impl Display for Attributes {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Luminous => "Luminous".to_string(),
            Self::Transparent { abs } => format!("Transparent: [{}]", abs),
            Self::Mirror { abs } => format!("Mirror: [{}]", abs),
            Self::Refractive {
                abs,
                inside,
                outside,
            } => format!("Refractive: [{}]\t{} :| {}", abs, inside, outside),
        };
        write!(fmt, "{}", kind)
    }
}
