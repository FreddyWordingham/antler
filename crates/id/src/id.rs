use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id<T> {
    index: usize,
    _marker: PhantomData<T>,
}

type Geometry = ();
pub type GeometryId = Id<Geometry>;

type Material = ();
pub type MaterialId = Id<Material>;

type Object = ();
pub type ObjectId = Id<Object>;

type Shader = ();
pub type ShaderId = Id<Shader>;

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
