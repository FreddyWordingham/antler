use crate::{aabb::Aabb, traversal_ray::TraversalRay};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BvhNode {
    pub aabb: Aabb,
    pub left_or_first: u32,
    pub right_or_count: u32,
    pub axis: u8,
    pub is_leaf: u8,
    pub _pad: [u8; 2],
}

impl BvhNode {
    #[inline]
    pub const fn is_leaf(&self) -> bool {
        self.is_leaf != 0
    }

    #[inline]
    pub const fn primitive_range(&self) -> std::ops::Range<usize> {
        let start = self.left_or_first as usize;
        let count = self.right_or_count as usize;
        start..start + count
    }

    #[inline]
    const fn left_child(&self) -> usize {
        self.left_or_first as usize
    }

    #[inline]
    const fn right_child(&self) -> usize {
        self.right_or_count as usize
    }

    #[inline]
    pub const fn near_far_children(&self, tray: &TraversalRay) -> (usize, usize) {
        let left = self.left_child();
        let right = self.right_child();

        if tray.dir_non_negative[self.axis as usize] {
            (left, right)
        } else {
            (right, left)
        }
    }

    #[inline]
    pub fn ray_interval(&self, tray: &TraversalRay) -> Option<(f32, f32)> {
        let tx0 = (self.aabb.min.x - tray.origin.x) * tray.inv_dir.x;
        let tx1 = (self.aabb.max.x - tray.origin.x) * tray.inv_dir.x;
        let mut t_min = tx0.min(tx1);
        let mut t_max = tx0.max(tx1);

        let ty0 = (self.aabb.min.y - tray.origin.y) * tray.inv_dir.y;
        let ty1 = (self.aabb.max.y - tray.origin.y) * tray.inv_dir.y;
        t_min = t_min.max(ty0.min(ty1));
        t_max = t_max.min(ty0.max(ty1));

        let tz0 = (self.aabb.min.z - tray.origin.z) * tray.inv_dir.z;
        let tz1 = (self.aabb.max.z - tray.origin.z) * tray.inv_dir.z;
        t_min = t_min.max(tz0.min(tz1));
        t_max = t_max.min(tz0.max(tz1));

        if t_max < t_min { None } else { Some((t_min, t_max)) }
    }
}
