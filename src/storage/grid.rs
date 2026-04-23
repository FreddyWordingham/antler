use std::ops::{Index, IndexMut};

pub struct Grid<T> {
    size: [usize; 2],
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_elem(size: [usize; 2], elem: T) -> Self
    where
        T: Clone,
    {
        let data = vec![elem; size[0] * size[1]];
        Self { size, data }
    }

    pub fn from_vec(size: [usize; 2], data: Vec<T>) -> Self {
        assert_eq!(
            data.len(),
            size[0] * size[1],
            "Data length ({}) must match size area ({})",
            data.len(),
            size[0] * size[1]
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
    pub const fn size(&self) -> [usize; 2] {
        self.size
    }

    #[inline]
    pub const fn width(&self) -> usize {
        self.size[0]
    }

    #[inline]
    pub const fn height(&self) -> usize {
        self.size[1]
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    #[inline]
    fn index_of(&self, coord: [usize; 2]) -> usize {
        assert!(
            coord[0] < self.size[0],
            "X coordinate ({}) must be less than width ({})",
            coord[0],
            self.size[0]
        );
        assert!(
            coord[1] < self.size[1],
            "Y coordinate ({}) must be less than height ({})",
            coord[1],
            self.size[1]
        );
        coord[1] * self.size[0] + coord[0]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, coord: (usize, usize)) -> &Self::Output {
        &self.data[self.index_of([coord.0, coord.1])]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, coord: (usize, usize)) -> &mut Self::Output {
        let index = self.index_of([coord.0, coord.1]);
        &mut self.data[index]
    }
}
