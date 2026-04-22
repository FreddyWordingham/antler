use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeometryId(usize);

impl Deref for GeometryId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
