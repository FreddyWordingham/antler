use std::f32::INFINITY;

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
        self.trace_node(0, ray, best_distance, &mut visit);
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
        self.trace_any_node(0, ray, max_distance, &mut test)
    }

    #[inline]
    pub fn trace_any_filtered<F>(&self, ray: &Ray, max_distance: f32, mut test: F) -> bool
    where
        F: FnMut(T) -> bool,
    {
        let mut max_distance = max_distance;
        self.trace_any_node(0, ray, &mut max_distance, &mut |id, _max_distance| test(id))
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

        items
            .sort_by(|(aabb_a, _), (aabb_b, _)| aabb_a.centroid()[axis].partial_cmp(&aabb_b.centroid()[axis]).unwrap());

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

    fn trace_node<F>(&self, node_index: usize, ray: &Ray, best_distance: &mut f32, visit: &mut F) -> bool
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let node = &self.nodes[node_index];
        let Some((node_t_min, _)) = node.aabb.ray_interval(ray) else {
            return true;
        };

        if node_t_min > *best_distance {
            return true;
        }

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
                        let (first, second, second_t) = if left_t <= right_t {
                            (left, right, right_t)
                        } else {
                            (right, left, left_t)
                        };

                        if !self.trace_node(first, ray, best_distance, visit) {
                            return false;
                        }

                        if second_t <= *best_distance && !self.trace_node(second, ray, best_distance, visit) {
                            return false;
                        }

                        true
                    }
                    (Some(_), None) => self.trace_node(left, ray, best_distance, visit),
                    (None, Some(_)) => self.trace_node(right, ray, best_distance, visit),
                    (None, None) => true,
                }
            }
        }
    }

    fn trace_any_node<F>(&self, node_index: usize, ray: &Ray, max_distance: &mut f32, test: &mut F) -> bool
    where
        F: FnMut(T, &mut f32) -> bool,
    {
        let node = &self.nodes[node_index];
        let Some((node_t_min, _)) = node.aabb.ray_interval(ray) else {
            return false;
        };

        if node_t_min > *max_distance {
            return false;
        }

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
                        let (first, second, second_t) = if left_t <= right_t {
                            (left, right, right_t)
                        } else {
                            (right, left, left_t)
                        };

                        if self.trace_any_node(first, ray, max_distance, test) {
                            return true;
                        }

                        if second_t <= *max_distance && self.trace_any_node(second, ray, max_distance, test) {
                            return true;
                        }

                        false
                    }

                    (Some(_), None) => self.trace_any_node(left, ray, max_distance, test),
                    (None, Some(_)) => self.trace_any_node(right, ray, max_distance, test),
                    (None, None) => false,
                }
            }
        }
    }
}
