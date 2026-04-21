pub use crate::{geometry::Ray, tracing::Hit};

pub trait Traceable {
    fn trace(&self, ray: &Ray) -> Option<Hit>;
}
