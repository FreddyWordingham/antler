pub use crate::tracing::{ObjectHit, ObjectRay};

pub trait Traceable {
    fn trace(&self, ray: &ObjectRay) -> Option<ObjectHit>;

    #[inline]
    fn trace_distance(&self, ray: &ObjectRay) -> Option<f32> {
        self.trace(ray).map(|hit| hit.distance)
    }
}
