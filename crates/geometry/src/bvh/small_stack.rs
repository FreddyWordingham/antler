use crate::bvh::traversal_entry::TraversalEntry;

pub struct SmallStack<const N: usize> {
    entries: [TraversalEntry; N],
    len: usize,
}

impl<const N: usize> SmallStack<N> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            entries: [TraversalEntry {
                node_index: 0,
                t_min: 0.0,
            }; N],
            len: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, entry: TraversalEntry) {
        assert!(self.len < N, "BVH traversal stack overflow");
        self.entries[self.len] = entry;
        self.len += 1;
    }

    #[inline]
    pub const fn pop(&mut self) -> Option<TraversalEntry> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(self.entries[self.len])
        }
    }
}
