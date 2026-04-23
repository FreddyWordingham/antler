use std::f32::INFINITY;

use nalgebra::Point3;

use crate::geometry::{Aabb, Ray, TraversalRay};

const LEAF_SIZE: usize = 4;
const SAH_BUCKETS: usize = 12;
const TRAVERSAL_COST: f32 = 1.0;
const INTERSECTION_COST: f32 = 1.0;
const TRAVERSAL_STACK_SIZE: usize = 64;

#[repr(C)]
#[derive(Debug, Clone, Copy)]

struct PackedBvhNode {
    min: Point3<f32>,
    max: Point3<f32>,
    left_or_first: u32,
    right_or_count: u32,
    axis: u8,
    is_leaf: u8,
    _pad: [u8; 2],
}

impl PackedBvhNode {
    #[inline]
    fn is_leaf(&self) -> bool {
        self.is_leaf != 0
    }

    #[inline]
    fn primitive_range(&self) -> std::ops::Range<usize> {
        let start = self.left_or_first as usize;
        let count = self.right_or_count as usize;
        start..start + count
    }

    #[inline]
    fn left_child(&self) -> usize {
        self.left_or_first as usize
    }

    #[inline]
    fn right_child(&self) -> usize {
        self.right_or_count as usize
    }

    #[inline]
    fn near_far_children(&self, tray: &TraversalRay) -> (usize, usize) {
        let left = self.left_child();
        let right = self.right_child();

        if tray.dir_non_negative[self.axis as usize] {
            (left, right)
        } else {
            (right, left)
        }
    }

    #[inline]
    fn ray_interval(&self, tray: &TraversalRay) -> Option<(f32, f32)> {
        let tx0 = (self.min.x - tray.origin.x) * tray.inv_dir.x;
        let tx1 = (self.max.x - tray.origin.x) * tray.inv_dir.x;
        let mut t_min = tx0.min(tx1);
        let mut t_max = tx0.max(tx1);

        let ty0 = (self.min.y - tray.origin.y) * tray.inv_dir.y;
        let ty1 = (self.max.y - tray.origin.y) * tray.inv_dir.y;
        t_min = t_min.max(ty0.min(ty1));
        t_max = t_max.min(ty0.max(ty1));

        let tz0 = (self.min.z - tray.origin.z) * tray.inv_dir.z;
        let tz1 = (self.max.z - tray.origin.z) * tray.inv_dir.z;
        t_min = t_min.max(tz0.min(tz1));
        t_max = t_max.min(tz0.max(tz1));

        if t_max < t_min { None } else { Some((t_min, t_max)) }
    }
}

#[derive(Clone)]
struct Bucket {
    count: usize,
    bounds: Option<Aabb>,
}

impl Bucket {
    fn empty() -> Self {
        Self { count: 0, bounds: None }
    }

    fn add(&mut self, bounds: Aabb) {
        self.count += 1;
        self.bounds = Some(match self.bounds.take() {
            Some(existing) => Aabb::union([existing, bounds].into_iter()),
            None => bounds,
        });
    }
}

#[derive(Debug, Clone, Copy)]
struct TraversalEntry {
    node_index: usize,
    t_min: f32,
}

struct SmallStack<const N: usize> {
    entries: [TraversalEntry; N],
    len: usize,
}

impl<const N: usize> SmallStack<N> {
    #[inline]
    fn new() -> Self {
        Self {
            entries: [TraversalEntry {
                node_index: 0,
                t_min: 0.0,
            }; N],
            len: 0,
        }
    }

    #[inline]
    fn push(&mut self, entry: TraversalEntry) {
        assert!(self.len < N, "BVH traversal stack overflow");
        self.entries[self.len] = entry;
        self.len += 1;
    }

    #[inline]
    fn pop(&mut self) -> Option<TraversalEntry> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(self.entries[self.len])
        }
    }
}

pub struct Bvh<T: Copy> {
    nodes: Vec<PackedBvhNode>,
    primitive_ids: Vec<T>,
}

impl<T: Copy> Bvh<T> {
    pub fn new(aabbs: Vec<(Aabb, T)>) -> Self {
        assert!(!aabbs.is_empty(), "Cannot build a BVH with no items.");

        let mut items = aabbs;
        let mut nodes = Vec::with_capacity(items.len() * 2);
        let mut primitive_ids = Vec::with_capacity(items.len());

        Self::build_node(&mut nodes, &mut primitive_ids, &mut items);

        Self { nodes, primitive_ids }
    }

    #[inline]
    pub fn trace<F>(&self, ray: &Ray, visit: F)
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let mut best_distance = INFINITY;
        self.trace_nearest_with_max(ray, &mut best_distance, visit);
    }

    pub fn trace_nearest_with_max<F>(&self, ray: &Ray, best_distance: &mut f32, mut visit: F)
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let tray = TraversalRay::new(ray);

        let Some((root_t_min, _)) = self.nodes[0].ray_interval(&tray) else {
            return;
        };

        self.trace_node(0, &tray, best_distance, &mut visit, root_t_min);
    }

    #[inline]
    pub fn trace_any<F>(&self, ray: &Ray, test: F) -> bool
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let mut max_distance = INFINITY;
        self.trace_any_with_limit(ray, &mut max_distance, test)
    }

    pub fn trace_any_with_limit<F>(&self, ray: &Ray, max_distance: &mut f32, mut test: F) -> bool
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let tray = TraversalRay::new(ray);

        let Some((root_t_min, _)) = self.nodes[0].ray_interval(&tray) else {
            return false;
        };

        self.trace_any_node(0, &tray, max_distance, &mut test, root_t_min)
    }

    #[inline]
    pub fn trace_any_filtered<F>(&self, ray: &Ray, max_distance: f32, mut test: F) -> bool
    where
        F: FnMut(T) -> bool,
    {
        let tray = TraversalRay::new(ray);

        let Some((root_t_min, _)) = self.nodes[0].ray_interval(&tray) else {
            return false;
        };

        let mut max_distance = max_distance;
        self.trace_any_node(0, &tray, &mut max_distance, &mut |id, _| test(id), root_t_min)
    }

    fn build_node(nodes: &mut Vec<PackedBvhNode>, primitive_ids: &mut Vec<T>, items: &mut [(Aabb, T)]) -> usize {
        let node_aabb = Aabb::union(items.iter().map(|(aabb, _)| aabb.clone()));
        let node_index = nodes.len();

        nodes.push(PackedBvhNode {
            min: node_aabb.min,
            max: node_aabb.max,
            left_or_first: 0,
            right_or_count: 0,
            axis: 0,
            is_leaf: 0,
            _pad: [0; 2],
        });

        if items.len() <= LEAF_SIZE {
            return Self::make_leaf(nodes, primitive_ids, node_index, items);
        }

        let centroid_bounds = Aabb::union(items.iter().map(|(aabb, _)| {
            let c = aabb.centroid();
            Aabb::new(c, c)
        }));

        let centroid_extent = centroid_bounds.max - centroid_bounds.min;

        let mut best_axis = None;
        let mut best_bucket_split = 0;
        let mut best_cost = INFINITY;

        for axis in 0..3 {
            let axis_extent = centroid_extent[axis];
            if axis_extent <= 0.0 {
                continue;
            }

            let mut buckets = vec![Bucket::empty(); SAH_BUCKETS];

            for (bounds, _) in items.iter() {
                let centroid = bounds.centroid()[axis];
                let mut bucket = (((centroid - centroid_bounds.min[axis]) / axis_extent) * SAH_BUCKETS as f32) as usize;
                bucket = bucket.min(SAH_BUCKETS - 1);
                buckets[bucket].add(bounds.clone());
            }

            let mut left_counts = [0; SAH_BUCKETS - 1];
            let mut right_counts = [0; SAH_BUCKETS - 1];
            let mut left_bounds: [Option<Aabb>; SAH_BUCKETS - 1] = std::array::from_fn(|_| None);
            let mut right_bounds: [Option<Aabb>; SAH_BUCKETS - 1] = std::array::from_fn(|_| None);

            let mut count = 0;
            let mut bounds = None::<Aabb>;
            for i in 0..(SAH_BUCKETS - 1) {
                count += buckets[i].count;
                if let Some(b) = buckets[i].bounds.clone() {
                    bounds = Some(match bounds.take() {
                        Some(existing) => Aabb::union([existing, b].into_iter()),
                        None => b,
                    });
                }
                left_counts[i] = count;
                left_bounds[i] = bounds.clone();
            }

            count = 0;
            bounds = None;
            for i in (1..SAH_BUCKETS).rev() {
                count += buckets[i].count;
                if let Some(b) = buckets[i].bounds.clone() {
                    bounds = Some(match bounds.take() {
                        Some(existing) => Aabb::union([existing, b].into_iter()),
                        None => b,
                    });
                }
                right_counts[i - 1] = count;
                right_bounds[i - 1] = bounds.clone();
            }

            let parent_area = node_aabb.surface_area();

            for i in 0..(SAH_BUCKETS - 1) {
                let Some(left_b) = &left_bounds[i] else {
                    continue;
                };
                let Some(right_b) = &right_bounds[i] else {
                    continue;
                };

                let cost = TRAVERSAL_COST
                    + INTERSECTION_COST
                        * ((left_counts[i] as f32 * left_b.surface_area()
                            + right_counts[i] as f32 * right_b.surface_area())
                            / parent_area);

                if cost < best_cost {
                    best_cost = cost;
                    best_axis = Some(axis);
                    best_bucket_split = i;
                }
            }
        }

        let leaf_cost = INTERSECTION_COST * items.len() as f32;

        let Some(axis) = best_axis else {
            return Self::make_leaf(nodes, primitive_ids, node_index, items);
        };

        if best_cost >= leaf_cost {
            return Self::make_leaf(nodes, primitive_ids, node_index, items);
        }

        let axis_extent = centroid_extent[axis];
        let axis_min = centroid_bounds.min[axis];

        let mid = partition_in_place(items, |(bounds, _)| {
            let centroid = bounds.centroid()[axis];
            let mut bucket = (((centroid - axis_min) / axis_extent) * SAH_BUCKETS as f32) as usize;
            bucket = bucket.min(SAH_BUCKETS - 1);
            bucket <= best_bucket_split
        });

        if mid == 0 || mid == items.len() {
            items.sort_by(|(a, _), (b, _)| {
                a.centroid()[axis]
                    .partial_cmp(&b.centroid()[axis])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            let mid = items.len() / 2;
            let (left_items, right_items) = items.split_at_mut(mid);

            let left = Self::build_node(nodes, primitive_ids, left_items);
            let right = Self::build_node(nodes, primitive_ids, right_items);

            nodes[node_index].left_or_first = left as u32;
            nodes[node_index].right_or_count = right as u32;
            nodes[node_index].axis = axis as u8;
            nodes[node_index].is_leaf = 0;

            return node_index;
        }

        let (left_items, right_items) = items.split_at_mut(mid);
        let left = Self::build_node(nodes, primitive_ids, left_items);
        let right = Self::build_node(nodes, primitive_ids, right_items);

        nodes[node_index].left_or_first = left as u32;
        nodes[node_index].right_or_count = right as u32;
        nodes[node_index].axis = axis as u8;
        nodes[node_index].is_leaf = 0;

        node_index
    }

    fn make_leaf(
        nodes: &mut [PackedBvhNode],
        primitive_ids: &mut Vec<T>,
        node_index: usize,
        items: &[(Aabb, T)],
    ) -> usize {
        let start = primitive_ids.len();
        primitive_ids.extend(items.iter().map(|(_, id)| *id));

        nodes[node_index].left_or_first = start as u32;
        nodes[node_index].right_or_count = items.len() as u32;
        nodes[node_index].axis = 0;
        nodes[node_index].is_leaf = 1;

        node_index
    }

    fn trace_node<F>(
        &self,
        node_index: usize,
        tray: &TraversalRay,
        best_distance: &mut f32,
        visit: &mut F,
        node_t_min: f32,
    ) -> bool
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let mut stack = SmallStack::<TRAVERSAL_STACK_SIZE>::new();
        stack.push(TraversalEntry {
            node_index,
            t_min: node_t_min,
        });

        while let Some(TraversalEntry { node_index, t_min }) = stack.pop() {
            if t_min > *best_distance {
                continue;
            }

            let node = &self.nodes[node_index];

            if node.is_leaf() {
                for i in node.primitive_range() {
                    if !visit(self.primitive_ids[i], best_distance) {
                        return false;
                    }
                }
                continue;
            }

            let (near, far) = node.near_far_children(tray);

            let near_interval = self.nodes[near].ray_interval(tray);
            let far_interval = self.nodes[far].ray_interval(tray);

            if let Some((far_t, _)) = far_interval {
                if far_t <= *best_distance {
                    stack.push(TraversalEntry {
                        node_index: far,
                        t_min: far_t,
                    });
                }
            }

            if let Some((near_t, _)) = near_interval {
                if near_t <= *best_distance {
                    stack.push(TraversalEntry {
                        node_index: near,
                        t_min: near_t,
                    });
                }
            }
        }

        true
    }

    fn trace_any_node<F>(
        &self,
        node_index: usize,
        tray: &TraversalRay,
        max_distance: &mut f32,
        test: &mut F,
        node_t_min: f32,
    ) -> bool
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let mut stack = SmallStack::<TRAVERSAL_STACK_SIZE>::new();
        stack.push(TraversalEntry {
            node_index,
            t_min: node_t_min,
        });

        while let Some(TraversalEntry { node_index, t_min }) = stack.pop() {
            if t_min > *max_distance {
                continue;
            }

            let node = &self.nodes[node_index];

            if node.is_leaf() {
                for i in node.primitive_range() {
                    if test(self.primitive_ids[i], max_distance) {
                        return true;
                    }
                }
                continue;
            }

            let (near, far) = node.near_far_children(tray);

            let near_interval = self.nodes[near].ray_interval(tray);
            let far_interval = self.nodes[far].ray_interval(tray);

            if let Some((far_t, _)) = far_interval {
                if far_t <= *max_distance {
                    stack.push(TraversalEntry {
                        node_index: far,
                        t_min: far_t,
                    });
                }
            }

            if let Some((near_t, _)) = near_interval {
                if near_t <= *max_distance {
                    stack.push(TraversalEntry {
                        node_index: near,
                        t_min: near_t,
                    });
                }
            }
        }

        false
    }
}

fn partition_in_place<T>(items: &mut [T], mut pred: impl FnMut(&T) -> bool) -> usize {
    let mut i = 0;
    let mut j = items.len();

    while i < j {
        if pred(&items[i]) {
            i += 1;
        } else {
            j -= 1;
            items.swap(i, j);
        }
    }

    i
}
