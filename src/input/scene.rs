//! Rendering simulation input structure.

use crate::{input::Settings, parts::Attributes};
use arctk::geom::{Mesh, Tree};
use arctk::ord::Set;
use palette::{Gradient, LinSrgba};

/// Rendering main input structure.
pub struct Scene<'a, T: Ord> {
    /// Adaptive tree.
    pub tree: &'a Tree<'a, &'a T>,
    /// Engine settings.
    pub sett: &'a Settings,
    /// Surfaces.
    pub surfs: &'a Set<T, Mesh>,
    /// Attributes.
    pub attrs: &'a Set<T, Attributes>,
    /// Colours.
    pub cols: &'a Set<T, Gradient<LinSrgba>>,
}

impl<'a, T: Ord> Scene<'a, T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        tree: &'a Tree<'a, &T>,
        sett: &'a Settings,
        surfs: &'a Set<T, Mesh>,
        attrs: &'a Set<T, Attributes>,
        cols: &'a Set<T, Gradient<LinSrgba>>,
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
