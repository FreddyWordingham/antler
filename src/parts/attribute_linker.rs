//! Attributes implementation.

use crate::{
    err::Error,
    img::Gradient,
    ord::{Link, Set},
    sim::render::Attribute,
};
use arctk_attr::load;

/// Surface attribute setup.
#[load]
pub enum AttributeLinker {
    /// Opaque coloured surface.
    Opaque(String),
    /// Partially reflective mirror, absorption fraction.
    Mirror(String, f64),
    /// Partially transparent, absorption fraction.
    Transparent(String, f64),
    /// Refractive, absorption fraction, inside and outside refractive indices.
    Refractive(String, f64, [f64; 2]),
    /// Luminous surface, brightness multiplier.
    Luminous(String, f64),
}

impl<'a> Link<'a, Gradient> for AttributeLinker {
    type Inst = Attribute<'a>;

    #[inline]
    fn requires(&self) -> Vec<String> {
        match *self {
            Self::Opaque(ref grad)
            | Self::Mirror(ref grad, ..)
            | Self::Transparent(ref grad, ..)
            | Self::Refractive(ref grad, ..)
            | Self::Luminous(ref grad, ..) => vec![grad.clone()],
        }
    }

    #[inline]
    fn link(self, grads: &'a Set<Gradient>) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Opaque(ref grad) => Attribute::Opaque(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {}", grad)),
            ),
            Self::Mirror(ref grad, abs_frac) => Attribute::Mirror(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {}", grad)),
                abs_frac,
            ),
            Self::Transparent(ref grad, abs_frac) => Attribute::Transparent(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {}", grad)),
                abs_frac,
            ),
            Self::Refractive(ref grad, abs_frac, ref_indices) => Attribute::Refractive(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {}", grad)),
                abs_frac,
                ref_indices,
            ),
            Self::Luminous(ref grad, bright_mult) => Attribute::Luminous(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {}", grad)),
                bright_mult,
            ),
        })
    }
}
