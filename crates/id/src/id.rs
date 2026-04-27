use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id<T> {
    index: usize,
    _marker: PhantomData<T>,
}

type Object = ();
pub type ObjectId = Id<Object>;

impl<T> Id<T> {
    #[inline]
    pub const fn new(index: usize) -> Self {
        Self {
            index,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub const fn index(self) -> usize {
        self.index
    }
}
