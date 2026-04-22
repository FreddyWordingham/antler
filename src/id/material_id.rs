#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(usize);

impl MaterialId {
    #[inline]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    #[inline]
    pub const fn index(self) -> usize {
        self.0
    }
}
