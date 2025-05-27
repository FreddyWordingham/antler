//! Collision geometry trait.

use nalgebra::RealField;

use crate::geometry::Aabb;

/// Types implementing this type can be checked for collisions with an axis-aligned bounding box.
pub trait Bounded<T: RealField> {
    /// Get the axis-aligned bounding box of the geometry.
    #[must_use]
    fn aabb(&self) -> Aabb<T>;
}

/// Types implementing this trait can return an array of [`Aabb`]'s, accesses by index.
pub trait IndexedBounds<T: RealField> {
    fn indexed_aabb(&self, index: usize) -> Aabb<T>;
}

impl<T: RealField, B: Bounded<T>> IndexedBounds<T> for Vec<B> {
    #[inline]
    fn indexed_aabb(&self, index: usize) -> Aabb<T> {
        self[index].aabb()
    }
}
