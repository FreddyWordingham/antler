pub use crate::{hit::Hit, ray::Ray};

pub trait Traceable {
    fn distance(&self, ray: &Ray) -> Option<f32>;
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}
