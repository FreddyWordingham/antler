//! Rendering simulation input structure.

use crate::{
    render::{Attributes, Settings},
    tree::Cell,
    Mesh, Set,
};
use palette::{Gradient, LinSrgba};

/// Rendering main input structure.
pub struct Scene<'a> {
    /// Adaptive tree.
    pub tree: &'a Cell<'a>,
    /// Engine settings.
    pub sett: &'a Settings,
    /// Surfaces.
    pub surfs: &'a Set<Mesh>,
    /// Attributes.
    pub attrs: &'a Set<Attributes>,
    /// Colours.
    pub cols: &'a Set<Gradient<LinSrgba>>,
}

impl<'a> Scene<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        tree: &'a Cell<'a>,
        sett: &'a Settings,
        surfs: &'a Set<Mesh>,
        attrs: &'a Set<Attributes>,
        cols: &'a Set<Gradient<LinSrgba>>,
    ) -> Self {
        Self {
            tree,
            sett,
            surfs,
            attrs,
            cols,
        }
    }
}
