#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderId(usize);

impl ShaderId {
    #[inline]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    #[inline]
    pub const fn index(self) -> usize {
        self.0
    }
}
