pub use crate::{contact::Contact, ray::Ray};

pub trait Traceable {
    #[must_use]
    fn hit(&self, ray: &Ray, max_distance: f32) -> bool;

    #[must_use]
    fn distance(&self, ray: &Ray, max_distance: f32) -> Option<f32>;

    #[must_use]
    fn intersection(&self, ray: &Ray, max_distance: f32) -> Option<Contact>;
}
