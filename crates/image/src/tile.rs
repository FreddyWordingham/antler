#[derive(Clone, Copy)]
pub struct Tile {
    pub min: [usize; 2],
    pub max: [usize; 2],
}

impl Tile {
    #[must_use]
    #[inline]
    pub const fn new(min: [usize; 2], max: [usize; 2]) -> Self {
        assert!(min[0] < max[0], "Tile min x must be less than max x");
        assert!(min[1] < max[1], "Tile min y must be less than max y");
        Self { min, max }
    }

    #[must_use]
    #[inline]
    pub fn create_tiles(resolution: [usize; 2], tile_size: [usize; 2]) -> Vec<Self> {
        let [tiles_x, tiles_y] = Self::count_tiles(resolution, tile_size);

        (0..tiles_y)
            .flat_map(|ty| {
                (0..tiles_x).map(move |tx| {
                    let min_x = tx * tile_size[0];
                    let min_y = ty * tile_size[1];
                    let max_x = (min_x + tile_size[0]).min(resolution[0]);
                    let max_y = (min_y + tile_size[1]).min(resolution[1]);

                    Self::new([min_x, min_y], [max_x, max_y])
                })
            })
            .collect()
    }

    #[must_use]
    #[inline]
    pub const fn count_tiles(resolution: [usize; 2], tile_size: [usize; 2]) -> [usize; 2] {
        let tiles_x = resolution[0].div_ceil(tile_size[0]);
        let tiles_y = resolution[1].div_ceil(tile_size[1]);
        [tiles_x, tiles_y]
    }

    #[must_use]
    #[inline]
    pub const fn size(&self) -> [usize; 2] {
        [self.max[0] - self.min[0], self.max[1] - self.min[1]]
    }

    #[must_use]
    #[inline]
    pub const fn num_pixels(&self) -> usize {
        let [width, height] = self.size();
        width * height
    }
}
