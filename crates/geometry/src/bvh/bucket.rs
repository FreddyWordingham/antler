use crate::aabb::Aabb;

#[derive(Clone)]
pub struct Bucket {
    pub count: usize,
    pub bounds: Option<Aabb>,
}

impl Bucket {
    #[inline]
    pub const fn empty() -> Self {
        Self { count: 0, bounds: None }
    }

    #[inline]
    pub fn add(&mut self, bounds: Aabb) {
        self.count += 1;
        self.bounds = Some(
            self.bounds
                .take()
                .map_or(bounds, |existing| Aabb::union([existing, bounds].into_iter())),
        );
    }
}
