use std::{cmp::Ordering, f32::INFINITY};

use crate::geometry::{Aabb, Ray};

const LEAF_SIZE: usize = 4;
const SAH_BUCKETS: usize = 12;
const TRAVERSAL_COST: f32 = 1.0;
const INTERSECTION_COST: f32 = 1.0;

pub struct Bvh<T: Copy> {
    nodes: Vec<BvhNode>,
    ids: Vec<T>,
}

struct BvhNode {
    aabb: Aabb,
    kind: BvhNodeKind,
}

enum BvhNodeKind {
    Branch { left: usize, right: usize },
    Leaf { start: usize, count: usize },
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

impl<T: Copy> Bvh<T> {
    pub fn new(aabbs: Vec<(Aabb, T)>) -> Self {
        assert!(!aabbs.is_empty(), "Cannot build a BVH with no items.");

        let mut items = aabbs;
        let mut nodes = Vec::new();
        let mut ids = Vec::new();

        Self::build_node(&mut nodes, &mut ids, &mut items);

        Self { nodes, ids }
    }

    #[inline]
    pub fn trace<F>(&self, ray: &Ray, visit: F)
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let mut best_distance = INFINITY;
        self.trace_nearest_with_max(ray, &mut best_distance, visit);
    }

    #[inline]
    pub fn trace_nearest_with_max<F>(&self, ray: &Ray, best_distance: &mut f32, mut visit: F)
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let Some((root_t_min, _)) = self.nodes[0].aabb.ray_interval(ray) else {
            return;
        };

        self.trace_node(0, ray, best_distance, &mut visit, root_t_min);
    }

    #[inline]
    pub fn trace_any<F>(&self, ray: &Ray, test: F) -> bool
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let mut max_distance = INFINITY;
        self.trace_any_with_limit(ray, &mut max_distance, test)
    }

    #[inline]
    pub fn trace_any_with_limit<F>(&self, ray: &Ray, max_distance: &mut f32, mut test: F) -> bool
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let Some((root_t_min, _)) = self.nodes[0].aabb.ray_interval(ray) else {
            return false;
        };

        self.trace_any_node(0, ray, max_distance, &mut test, root_t_min)
    }

    #[inline]
    pub fn trace_any_filtered<F>(&self, ray: &Ray, max_distance: f32, mut test: F) -> bool
    where
        F: FnMut(T) -> bool,
    {
        let Some((root_t_min, _)) = self.nodes[0].aabb.ray_interval(ray) else {
            return false;
        };

        let mut max_distance = max_distance;
        self.trace_any_node(0, ray, &mut max_distance, &mut |id, _| test(id), root_t_min)
    }

    fn build_node(nodes: &mut Vec<BvhNode>, ids: &mut Vec<T>, items: &mut [(Aabb, T)]) -> usize {
        let node_index = nodes.len();
        let node_aabb = Aabb::union(items.iter().map(|(aabb, _)| aabb.clone()));

        nodes.push(BvhNode {
            aabb: node_aabb.clone(),
            kind: BvhNodeKind::Leaf { start: 0, count: 0 },
        });

        if items.len() <= LEAF_SIZE {
            return Self::make_leaf(nodes, ids, node_index, items);
        }

        let centroid_bounds = Aabb::union(items.iter().map(|(aabb, _)| {
            let c = aabb.centroid();
            Aabb::new(c, c)
        }));

        let centroid_extent = centroid_bounds.max - centroid_bounds.min;

        let mut best_axis = None;
        let mut best_bucket_split = 0usize;
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

            let mut left_counts = [0usize; SAH_BUCKETS - 1];
            let mut right_counts = [0usize; SAH_BUCKETS - 1];
            let mut left_bounds: [Option<Aabb>; SAH_BUCKETS - 1] = std::array::from_fn(|_| None);
            let mut right_bounds: [Option<Aabb>; SAH_BUCKETS - 1] = std::array::from_fn(|_| None);

            let mut count = 0usize;
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

            for i in 0..(SAH_BUCKETS - 1) {
                let Some(left_b) = &left_bounds[i] else {
                    continue;
                };
                let Some(right_b) = &right_bounds[i] else {
                    continue;
                };

                let left_area = left_b.surface_area();
                let right_area = right_b.surface_area();
                let parent_area = node_aabb.surface_area();

                let cost = TRAVERSAL_COST
                    + INTERSECTION_COST
                        * ((left_counts[i] as f32 * left_area + right_counts[i] as f32 * right_area) / parent_area);

                if cost < best_cost {
                    best_cost = cost;
                    best_axis = Some(axis);
                    best_bucket_split = i;
                }
            }
        }

        let leaf_cost = INTERSECTION_COST * items.len() as f32;

        let Some(axis) = best_axis else {
            return Self::make_leaf(nodes, ids, node_index, items);
        };

        if best_cost >= leaf_cost {
            return Self::make_leaf(nodes, ids, node_index, items);
        }

        let axis_extent = centroid_extent[axis];
        let min_axis = centroid_bounds.min[axis];

        let mid = partition_in_place(items, |(bounds, _)| {
            let centroid = bounds.centroid()[axis];
            let mut bucket = (((centroid - min_axis) / axis_extent) * SAH_BUCKETS as f32) as usize;
            bucket = bucket.min(SAH_BUCKETS - 1);
            bucket <= best_bucket_split
        });

        if mid == 0 || mid == items.len() {
            items.sort_by(|(a, _), (b, _)| {
                a.centroid()[axis]
                    .partial_cmp(&b.centroid()[axis])
                    .unwrap_or(Ordering::Equal)
            });

            let mid = items.len() / 2;
            let (left_items, right_items) = items.split_at_mut(mid);

            let left = Self::build_node(nodes, ids, left_items);
            let right = Self::build_node(nodes, ids, right_items);

            nodes[node_index] = BvhNode {
                aabb: node_aabb,
                kind: BvhNodeKind::Branch { left, right },
            };

            return node_index;
        }

        let (left_items, right_items) = items.split_at_mut(mid);

        let left = Self::build_node(nodes, ids, left_items);
        let right = Self::build_node(nodes, ids, right_items);

        nodes[node_index] = BvhNode {
            aabb: node_aabb,
            kind: BvhNodeKind::Branch { left, right },
        };

        node_index
    }

    fn make_leaf(nodes: &mut [BvhNode], ids: &mut Vec<T>, node_index: usize, items: &[(Aabb, T)]) -> usize {
        let start = ids.len();
        ids.extend(items.iter().map(|(_, id)| *id));
        let count = items.len();

        nodes[node_index] = BvhNode {
            aabb: nodes[node_index].aabb.clone(),
            kind: BvhNodeKind::Leaf { start, count },
        };

        node_index
    }

    fn trace_node<F>(
        &self,
        node_index: usize,
        ray: &Ray,
        best_distance: &mut f32,
        visit: &mut F,
        node_t_min: f32,
    ) -> bool
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        if node_t_min > *best_distance {
            return true;
        }

        let node = &self.nodes[node_index];

        match node.kind {
            BvhNodeKind::Leaf { start, count } => {
                for id in &self.ids[start..start + count] {
                    if !visit(*id, best_distance) {
                        return false;
                    }
                }
                true
            }
            BvhNodeKind::Branch { left, right } => {
                let left_interval = self.nodes[left].aabb.ray_interval(ray);
                let right_interval = self.nodes[right].aabb.ray_interval(ray);

                match (left_interval, right_interval) {
                    (Some((left_t, _)), Some((right_t, _))) => {
                        let (first, first_t, second, second_t) = if left_t <= right_t {
                            (left, left_t, right, right_t)
                        } else {
                            (right, right_t, left, left_t)
                        };

                        if !self.trace_node(first, ray, best_distance, visit, first_t) {
                            return false;
                        }

                        if *best_distance < second_t {
                            return true;
                        }

                        self.trace_node(second, ray, best_distance, visit, second_t)
                    }
                    (Some((left_t, _)), None) => self.trace_node(left, ray, best_distance, visit, left_t),
                    (None, Some((right_t, _))) => self.trace_node(right, ray, best_distance, visit, right_t),
                    (None, None) => true,
                }
            }
        }
    }

    fn trace_any_node<F>(
        &self,
        node_index: usize,
        ray: &Ray,
        max_distance: &mut f32,
        test: &mut F,
        node_t_min: f32,
    ) -> bool
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        if node_t_min > *max_distance {
            return false;
        }

        let node = &self.nodes[node_index];

        match node.kind {
            BvhNodeKind::Leaf { start, count } => {
                for id in &self.ids[start..start + count] {
                    if test(*id, max_distance) {
                        return true;
                    }
                }
                false
            }
            BvhNodeKind::Branch { left, right } => {
                let left_interval = self.nodes[left].aabb.ray_interval(ray);
                let right_interval = self.nodes[right].aabb.ray_interval(ray);

                match (left_interval, right_interval) {
                    (Some((left_t, _)), Some((right_t, _))) => {
                        let (first, first_t, second, second_t) = if left_t <= right_t {
                            (left, left_t, right, right_t)
                        } else {
                            (right, right_t, left, left_t)
                        };

                        if self.trace_any_node(first, ray, max_distance, test, first_t) {
                            return true;
                        }

                        if *max_distance < second_t {
                            return false;
                        }

                        self.trace_any_node(second, ray, max_distance, test, second_t)
                    }
                    (Some((left_t, _)), None) => self.trace_any_node(left, ray, max_distance, test, left_t),
                    (None, Some((right_t, _))) => self.trace_any_node(right, ray, max_distance, test, right_t),
                    (None, None) => false,
                }
            }
        }
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
