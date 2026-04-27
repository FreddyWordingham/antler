pub use crate::{intersection::Intersection, ray::Ray};

pub trait Traceable {
    #[must_use]
    fn distance(&self, ray: &Ray) -> Option<f32>;

    #[must_use]
    fn intersection(&self, ray: &Ray) -> Option<Intersection>;
}
