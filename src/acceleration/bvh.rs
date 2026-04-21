use crate::{geometry::Aabb, id::ObjectId};

pub struct Bvh {
    nodes: Vec<BvhNode>,
    object_ids: Vec<ObjectId>,
}

struct BvhNode {
    aabb: Aabb,
    kind: BvhNodeKind,
}

enum BvhNodeKind {
    Branch { left: usize, right: usize },
    Leaf { start: usize, count: usize },
}
