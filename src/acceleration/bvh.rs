use std::{cmp::Ordering, f32::INFINITY};

use crate::geometry::{Aabb, Ray};

const LEAF_SIZE: usize = 4;

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
        self.trace_any_node(0, ray, &mut max_distance, &mut |id, _max_distance| test(id), root_t_min)
    }

    fn build_node(nodes: &mut Vec<BvhNode>, ids: &mut Vec<T>, items: &mut [(Aabb, T)]) -> usize {
        let node_index = nodes.len();

        let node_aabb = Aabb::union(items.iter().map(|(aabb, _)| aabb.clone()));

        nodes.push(BvhNode {
            aabb: node_aabb,
            kind: BvhNodeKind::Leaf { start: 0, count: 0 },
        });

        if items.len() <= LEAF_SIZE {
            let start = ids.len();
            ids.extend(items.iter().map(|(_, id)| *id));
            let count = items.len();

            nodes[node_index] = BvhNode {
                aabb: nodes[node_index].aabb.clone(),
                kind: BvhNodeKind::Leaf { start, count },
            };

            return node_index;
        }

        let extent = nodes[node_index].aabb.max - nodes[node_index].aabb.min;
        let axis = if extent.x >= extent.y && extent.x >= extent.z {
            0
        } else if extent.y >= extent.z {
            1
        } else {
            2
        };

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
            aabb: nodes[node_index].aabb.clone(),
            kind: BvhNodeKind::Branch { left, right },
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
