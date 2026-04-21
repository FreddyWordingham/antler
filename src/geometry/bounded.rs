use crate::geometry::Aabb;

pub trait Bounded {
    fn bounds(&self) -> Aabb;
}
