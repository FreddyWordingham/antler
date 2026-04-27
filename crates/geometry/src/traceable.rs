pub use crate::{hit::Hit, ray::Ray};

pub trait Traceable {
    #[must_use]
    fn distance(&self, ray: &Ray) -> Option<f32>;

    #[must_use]
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}
