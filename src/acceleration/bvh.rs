use crate::geometry::Aabb;

pub struct Bvh<T> {
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
