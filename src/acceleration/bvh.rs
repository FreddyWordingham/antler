use crate::{geometry::Aabb, tracing::WorldRay};

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

    pub fn trace(&self, ray: &WorldRay) -> Vec<T> {
        let mut hits = Vec::new();
        self.trace_node(0, ray, &mut hits);
        hits
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

    fn trace_node(&self, node_index: usize, ray: &WorldRay, hits: &mut Vec<T>) {
        let node = &self.nodes[node_index];

        if node.aabb.ray_interval(ray).is_none() {
            return;
        }

        match node.kind {
            BvhNodeKind::Branch { left, right } => {
                let left_interval = self.nodes[left].aabb.ray_interval(ray);
                let right_interval = self.nodes[right].aabb.ray_interval(ray);

                match (left_interval, right_interval) {
                    (Some((left_t, _)), Some((right_t, _))) => {
                        if left_t <= right_t {
                            self.trace_node(left, ray, hits);
                            self.trace_node(right, ray, hits);
                        } else {
                            self.trace_node(right, ray, hits);
                            self.trace_node(left, ray, hits);
                        }
                    }

                    (Some(_), None) => {
                        self.trace_node(left, ray, hits);
                    }

                    (None, Some(_)) => {
                        self.trace_node(right, ray, hits);
                    }

                    (None, None) => {}
                }
            }

            BvhNodeKind::Leaf { start, count } => {
                hits.extend_from_slice(&self.ids[start..start + count]);
            }
        }
    }
}
