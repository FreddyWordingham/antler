use crate::aabb::Aabb;

pub trait Bounded {
    fn bounds(&self) -> Aabb;
}
