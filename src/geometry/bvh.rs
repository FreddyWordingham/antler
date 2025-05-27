//! Bounding Volume Hierarchy node structure.

use nalgebra::{Point3, RealField};

use crate::geometry::{Aabb, Bounded, IndexedBounds, Ray};

/// Bounding volume hierarchy node.
#[derive(Clone)]
pub struct BvhNode<T: RealField> {
    /// Bounding box.
    pub aabb: Aabb<T>,
    /// Left child node index. Right child node index is `left_child + 1`.
    pub left_child: usize,
    /// Number of objects contained in this node.
    pub count: usize,
}

pub struct BvhBuilder<T: RealField> {
    /// Indices of shapes contained in this node.
    indices: Vec<usize>,
    /// List of nodes.
    nodes: Vec<BvhNode<T>>,
    /// Current number of nodes used.
    nodes_used: usize,
}

impl<T: RealField> BvhBuilder<T> {
    /// Construct a new `BvhBuilder` instance.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            nodes: Vec::new(),
            nodes_used: 0,
        }
    }

    /// Generate a `Bvh` from a collection of bounded shapes.
    #[must_use]
    #[inline]
    pub fn build<B: Bounded<T>>(mut self, shapes: &[B], max_children: usize, max_depth: usize) -> Bvh<T> {
        debug_assert!(!shapes.is_empty(), "BVH must contain at least one geometry");
        debug_assert!(max_children >= 2, "BVH max children must be greater than two");
        debug_assert!(max_depth > 0, "BVH max depth must be positive");

        self.indices = (0..shapes.len()).collect();
        self.nodes = vec![
            BvhNode {
                aabb: Aabb::new_unchecked(
                    Point3::new(T::max_value().unwrap(), T::max_value().unwrap(), T::max_value().unwrap()),
                    Point3::new(T::min_value().unwrap(), T::min_value().unwrap(), T::min_value().unwrap())
                ),
                left_child: 0,
                count: 0,
            };
            (shapes.len() * 2) - 1
        ];

        self.nodes[0].left_child = 0;
        self.nodes[0].count = shapes.len();
        self.nodes_used = 1;

        self.update_bounds(0, shapes);
        let depth = self.subdivide(0, shapes, max_children, max_depth, 0);

        self.nodes.truncate(self.nodes_used);
        self.nodes.shrink_to_fit();

        Bvh {
            indices: self.indices,
            nodes: self.nodes,
            depth,
        }
    }

    /// Expand the bounding box of a `BvhNode` to include all geometries contained within the node.
    #[inline]
    fn update_bounds<B: Bounded<T>>(&mut self, index: usize, shapes: &[B]) {
        self.nodes[index].aabb = (0..self.nodes[index].count)
            .map(|i| shapes[self.indices[self.nodes[index].left_child + i]].aabb())
            .fold(self.nodes[index].aabb.clone(), |acc, aabb| acc.union(&aabb));
    }

    /// Subdivide a `BvhNode` into two child nodes if it contains more than the maximum number of allowed children and has not reached the maximum depth.
    #[inline]
    fn subdivide<B: Bounded<T>>(
        &mut self,
        index: usize,
        shapes: &[B],
        max_children: usize,
        max_depth: usize,
        current_depth: usize,
    ) -> usize {
        debug_assert!(max_children >= 2, "BVH max children must be greater than two");

        if (self.nodes[index].count <= max_children) || (current_depth >= max_depth) {
            return current_depth;
        }

        let extent = [
            self.nodes[index].aabb.maxs[0].clone() - self.nodes[index].aabb.mins[0].clone(),
            self.nodes[index].aabb.maxs[1].clone() - self.nodes[index].aabb.mins[1].clone(),
            self.nodes[index].aabb.maxs[2].clone() - self.nodes[index].aabb.mins[2].clone(),
        ];
        let axis = if extent[0] > extent[1] && extent[0] > extent[2] {
            0
        } else if extent[1] > extent[2] {
            1
        } else {
            2
        };

        let split_position = extent[axis]
            .clone()
            .mul_add(T::from_f32(0.5).unwrap(), self.nodes[index].aabb.mins[axis].clone());

        let mut i = self.nodes[index].left_child;
        let mut j = i + self.nodes[index].count - 1;

        while i <= j {
            if shapes[self.indices[i]].aabb().centre()[axis] < split_position {
                i += 1;
            } else {
                self.indices.swap(i, j);
                if j == 0 {
                    return current_depth;
                }
                j -= 1;
            }
        }

        let left_count = i - self.nodes[index].left_child;
        if (left_count == 0) || (left_count == self.nodes[index].count) {
            return current_depth;
        }

        let left_child_index = self.nodes_used;
        self.nodes_used += 1;
        let right_child_index = self.nodes_used;
        self.nodes_used += 1;

        self.nodes[left_child_index].left_child = self.nodes[index].left_child;
        self.nodes[left_child_index].count = left_count;

        self.nodes[right_child_index].left_child = i;
        self.nodes[right_child_index].count = self.nodes[index].count - left_count;

        self.nodes[index].left_child = left_child_index;
        self.nodes[index].count = 0;

        self.update_bounds(left_child_index, shapes);
        self.update_bounds(right_child_index, shapes);
        let left_depth = self.subdivide(left_child_index, shapes, max_children, max_depth, current_depth + 1);
        let right_depth = self.subdivide(right_child_index, shapes, max_children, max_depth, current_depth + 1);

        left_depth.max(right_depth)
    }
}

/// Bounding volume hierarchy.
pub struct Bvh<T: RealField> {
    /// Indices of objects contained in this node.
    indices: Vec<usize>,
    /// List of nodes.
    nodes: Vec<BvhNode<T>>,
    /// Depth of the tree.
    depth: usize,
}

impl<T: RealField> Bvh<T> {
    /// Construct a new `Bvh` instance.
    #[inline]
    #[must_use]
    pub fn new<B: Bounded<T>>(shapes: &[B], max_children: usize, max_depth: usize) -> Self {
        BvhBuilder::new().build(shapes, max_children, max_depth)
    }

    /// Get the depth of the bounding volume hierarchy.
    #[must_use]
    #[inline]
    pub fn depth(&self) -> usize {
        self.depth
    }

    #[inline]
    pub fn ray_intersections<S: IndexedBounds<T>>(&self, ray: &Ray<T>, shapes: &S) -> Vec<(usize, T)> {
        let mut hits = Vec::new();
        self.ray_intersect_node(0, ray, shapes, &mut hits);
        // sort by entry‐distance
        hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        hits
    }

    #[inline]
    fn ray_intersect_node<S: IndexedBounds<T>>(&self, node_index: usize, ray: &Ray<T>, shapes: &S, hits: &mut Vec<(usize, T)>) {
        if let Some(_) = self.nodes[node_index].aabb.intersect_distance(ray) {
            let node = &self.nodes[node_index];
            if node.count == 0 {
                // internal node
                let lc = node.left_child;
                self.ray_intersect_node(lc, ray, shapes, hits);
                self.ray_intersect_node(lc + 1, ray, shapes, hits);
            } else {
                // leaf: pull each child AABB via the trait
                for i in 0..node.count {
                    let idx = self.indices[node.left_child + i];
                    let aabb = shapes.indexed_aabb(idx);
                    if let Some(dist) = aabb.intersect_distance(ray) {
                        hits.push((idx, dist));
                    }
                }
            }
        }
    }
}
