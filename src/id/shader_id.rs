use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderId(usize);

impl Deref for ShaderId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
