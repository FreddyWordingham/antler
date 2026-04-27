use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Grid<T, const D: usize> {
    size: [usize; D],
    data: Vec<T>,
}

pub type LineGrid<T> = Grid<T, 1>;
pub type SurfaceGrid<T> = Grid<T, 2>;
pub type VolumeGrid<T> = Grid<T, 3>;
pub type HyperGrid<T> = Grid<T, 4>;

impl<T, const D: usize> Grid<T, D> {
    pub fn from_elem(size: [usize; D], elem: T) -> Self
    where
        T: Clone,
    {
        let data = vec![elem; size.iter().product()];
        Self { size, data }
    }

    pub fn from_vec(size: [usize; D], data: Vec<T>) -> Self {
        let expected_len = size.iter().product();

        assert_eq!(
            data.len(),
            expected_len,
            "Data length ({}) must match grid capacity ({})",
            data.len(),
            expected_len
        );

        Self { size, data }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub const fn size(&self) -> [usize; D] {
        self.size
    }

    #[inline]
    pub fn data(&self) -> &[T] {
        &self.data
    }

    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    #[inline]
    pub fn get(&self, coord: [usize; D]) -> &T {
        &self.data[self.index_of(coord)]
    }

    #[inline]
    pub fn get_mut(&mut self, coord: [usize; D]) -> &mut T {
        let index = self.index_of(coord);
        &mut self.data[index]
    }

    #[inline]
    fn index_of(&self, coord: [usize; D]) -> usize {
        let mut index = 0;
        let mut stride = 1;

        for (axis, (&c, &size)) in coord.iter().zip(self.size.iter()).enumerate() {
            assert!(
                c < size,
                "Coordinate on axis {} ({}) must be less than size ({})",
                axis,
                c,
                size
            );

            index += c * stride;
            stride *= size;
        }

        index
    }
}

impl<T, const D: usize> Index<[usize; D]> for Grid<T, D> {
    type Output = T;

    fn index(&self, coord: [usize; D]) -> &Self::Output {
        self.get(coord)
    }
}

impl<T, const D: usize> IndexMut<[usize; D]> for Grid<T, D> {
    fn index_mut(&mut self, coord: [usize; D]) -> &mut Self::Output {
        self.get_mut(coord)
    }
}
