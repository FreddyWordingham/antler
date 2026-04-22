use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(usize);

impl Deref for MaterialId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
