use std::ops::ControlFlow;

use crate::{
    aabb::Aabb,
    bvh::{bucket::Bucket, bvh_node::BvhNode, small_stack::SmallStack, traversal_entry::TraversalEntry},
    ray::Ray,
    traversal_ray::TraversalRay,
};

const LEAF_SIZE: usize = 4;
const SAH_BUCKETS: usize = 12;
const TRAVERSAL_COST: f32 = 1.0;
const INTERSECTION_COST: f32 = 1.0;
const TRAVERSAL_STACK_SIZE: usize = 64;

pub struct Bvh<T: Copy> {
    nodes: Vec<BvhNode>,
    primitive_ids: Vec<T>,
}

impl<T: Copy> Bvh<T> {
    #[must_use]
    pub fn new(aabbs: Vec<(Aabb, T)>) -> Self {
        assert!(!aabbs.is_empty(), "Cannot build a BVH with no items.");

        let mut items = aabbs;
        let mut nodes = Vec::with_capacity(items.len() * 2);
        let mut primitive_ids = Vec::with_capacity(items.len());

        Self::build_node(&mut nodes, &mut primitive_ids, &mut items);

        Self { nodes, primitive_ids }
    }

    pub fn nearest_with_max<F>(&self, ray: &Ray, best_distance: &mut f32, mut visit: F)
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let tray = TraversalRay::new(ray);

        let Some((root_t_min, _)) = self.nodes[0].ray_interval(&tray) else {
            return;
        };

        let mut stack = SmallStack::<TRAVERSAL_STACK_SIZE>::new();
        stack.push(TraversalEntry {
            node_index: 0,
            t_min: root_t_min,
        });

        while let Some(TraversalEntry { node_index, t_min }) = stack.pop() {
            if t_min > *best_distance {
                continue;
            }

            let node = &self.nodes[node_index];

            if node.is_leaf() {
                for i in node.primitive_range() {
                    if !visit(self.primitive_ids[i], best_distance) {
                        return;
                    }
                }
                continue;
            }

            let (near, far) = node.near_far_children(&tray);

            if let Some((far_t, _)) = self.nodes[far].ray_interval(&tray)
                && far_t <= *best_distance
            {
                stack.push(TraversalEntry {
                    node_index: far,
                    t_min: far_t,
                });
            }

            if let Some((near_t, _)) = self.nodes[near].ray_interval(&tray)
                && near_t <= *best_distance
            {
                stack.push(TraversalEntry {
                    node_index: near,
                    t_min: near_t,
                });
            }
        }
    }

    #[inline]
    pub fn any_with_limit<F>(&self, ray: &Ray, max_distance: &mut f32, mut test: F) -> bool
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let tray = TraversalRay::new(ray);

        let Some((root_t_min, _)) = self.nodes[0].ray_interval(&tray) else {
            return false;
        };

        self.traverse_any(0, root_t_min, &tray, max_distance, |primitive_range, max_distance| {
            for i in primitive_range {
                if test(self.primitive_ids[i], max_distance) {
                    return ControlFlow::Break(true);
                }
            }
            ControlFlow::Continue(())
        })
        .break_value()
        .unwrap_or(false)
    }

    #[allow(clippy::too_many_lines)]
    fn build_node(nodes: &mut Vec<BvhNode>, primitive_ids: &mut Vec<T>, items: &mut [(Aabb, T)]) -> usize {
        let node_aabb = Aabb::union(items.iter().map(|(aabb, _)| *aabb));
        let node_index = nodes.len();

        nodes.push(BvhNode {
            aabb: node_aabb,
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
        let mut best_cost = f32::INFINITY;

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
                buckets[bucket].add(*bounds);
            }

            let mut left_counts = [0; SAH_BUCKETS - 1];
            let mut right_counts = [0; SAH_BUCKETS - 1];
            let mut left_bounds: [Option<Aabb>; SAH_BUCKETS - 1] = std::array::from_fn(|_| None);
            let mut right_bounds: [Option<Aabb>; SAH_BUCKETS - 1] = std::array::from_fn(|_| None);

            let mut count = 0;
            let mut bounds = None::<Aabb>;
            for i in 0..(SAH_BUCKETS - 1) {
                count += buckets[i].count;
                if let Some(b) = buckets[i].bounds {
                    bounds = Some(
                        bounds
                            .take()
                            .map_or(b, |existing| Aabb::union([existing, b].into_iter())),
                    );
                }
                left_counts[i] = count;
                left_bounds[i] = bounds;
            }

            count = 0;
            bounds = None;
            for i in (1..SAH_BUCKETS).rev() {
                count += buckets[i].count;
                if let Some(b) = buckets[i].bounds {
                    bounds = Some(
                        bounds
                            .take()
                            .map_or(b, |existing| Aabb::union([existing, b].into_iter())),
                    );
                }
                right_counts[i - 1] = count;
                right_bounds[i - 1] = bounds;
            }

            let parent_area = node_aabb.area();

            for i in 0..(SAH_BUCKETS - 1) {
                let Some(left_b) = &left_bounds[i] else {
                    continue;
                };
                let Some(right_b) = &right_bounds[i] else {
                    continue;
                };

                let cost = INTERSECTION_COST.mul_add(
                    (left_counts[i] as f32).mul_add(left_b.area(), right_counts[i] as f32 * right_b.area())
                        / parent_area,
                    TRAVERSAL_COST,
                );

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

    fn make_leaf(nodes: &mut [BvhNode], primitive_ids: &mut Vec<T>, node_index: usize, items: &[(Aabb, T)]) -> usize {
        let start = primitive_ids.len();
        primitive_ids.extend(items.iter().map(|(_, id)| *id));

        nodes[node_index].left_or_first = start as u32;
        nodes[node_index].right_or_count = items.len() as u32;
        nodes[node_index].axis = 0;
        nodes[node_index].is_leaf = 1;

        node_index
    }

    fn traverse_any<L>(
        &self,
        root_index: usize,
        root_t_min: f32,
        tray: &TraversalRay,
        max_distance: &mut f32,
        mut visit_leaf: L,
    ) -> ControlFlow<bool, ()>
    where
        L: FnMut(std::ops::Range<usize>, &mut f32) -> ControlFlow<bool, ()>,
    {
        let mut stack = SmallStack::<TRAVERSAL_STACK_SIZE>::new();
        stack.push(TraversalEntry {
            node_index: root_index,
            t_min: root_t_min,
        });

        while let Some(TraversalEntry { node_index, t_min }) = stack.pop() {
            if t_min > *max_distance {
                continue;
            }

            let node = &self.nodes[node_index];

            if node.is_leaf() {
                match visit_leaf(node.primitive_range(), max_distance) {
                    ControlFlow::Break(value) => return ControlFlow::Break(value),
                    ControlFlow::Continue(()) => continue,
                }
            }

            let (near, far) = node.near_far_children(tray);

            if let Some((far_t, _)) = self.nodes[far].ray_interval(tray)
                && far_t <= *max_distance
            {
                stack.push(TraversalEntry {
                    node_index: far,
                    t_min: far_t,
                });
            }

            if let Some((near_t, _)) = self.nodes[near].ray_interval(tray)
                && near_t <= *max_distance
            {
                stack.push(TraversalEntry {
                    node_index: near,
                    t_min: near_t,
                });
            }
        }

        ControlFlow::Continue(())
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
