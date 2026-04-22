pub use crate::tracing::{ObjectHit, ObjectRay};

pub trait Traceable {
    fn trace(&self, ray: &ObjectRay) -> Option<ObjectHit>;
}
