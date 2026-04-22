use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(usize);

impl Deref for ObjectId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
