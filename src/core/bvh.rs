use std::ops::RangeInclusive;

use crate::{
    core::aabb::AxisAlignedBoundingBox,
    object::{
        hittable_object::{HitRecord, HittableObject},
        HittableObjectType,
    },
    ray::Ray,
};

pub struct BvhTree {
    /// Collection of all nodes in a tree. Root is always at index 0.
    nodes: Vec<BvhValue>,
}

impl BvhTree {
    fn root(&self) -> &BvhValue {
        &self.nodes[0]
    }

    fn hit_node(
        &self,
        node: &BvhValue,
        ray: &Ray,
        t_range: &RangeInclusive<f64>,
    ) -> Option<HitRecord> {
        match node {
            BvhValue::Node(node) => {
                if !node.bounding_box().intersects_ray(ray, t_range) {
                    return None;
                }

                let hit_left = self.hit_node(&self.nodes[node.left_id], ray, t_range);

                let hit_right_max_t = if let Some(hr) = &hit_left {
                    hr.t()
                } else {
                    *t_range.end()
                };

                let hit_right_range = *t_range.start()..=hit_right_max_t;
                let hit_right = self.hit_node(&self.nodes[node.right_id], ray, &hit_right_range);
                if hit_right.is_some() {
                    return hit_right;
                }

                hit_left
            }
            BvhValue::Leaf(leaf) => leaf.hit(ray, t_range),
        }
    }

    /// `end` is exclusive
    fn split_into_nodes(
        nodes: &mut Vec<BvhValue>,
        objects: &mut [HittableObjectType],
        start: usize,
        end: usize,
    ) -> (usize, usize) {
        let diff = end - start;

        if diff == 1 {
            let leaf = BvhLeaf {
                object: objects[start].clone(),
            };
            nodes.push(BvhValue::Leaf(leaf));
            let id = nodes.len() - 1;
            return (id, id);
        }

        if start + 1 == end {
            let left_leaf = BvhLeaf {
                object: objects[start].clone(),
            };
            nodes.push(BvhValue::Leaf(left_leaf));
            let right_leaf = BvhLeaf {
                object: objects[end].clone(),
            };
            nodes.push(BvhValue::Leaf(right_leaf));
            let len = nodes.len();
            return (len - 2, len - 1);
        }

        let mut bounding_box = AxisAlignedBoundingBox::empty();

        for obj in objects.iter().take(end).skip(start) {
            bounding_box = AxisAlignedBoundingBox::merge(&bounding_box, obj.bounding_box());
        }

        let longest_axis = bounding_box.longest_axis();
        objects[start..end].sort_by(|a, b| {
            AxisAlignedBoundingBox::compare_by_axis(
                a.bounding_box(),
                b.bounding_box(),
                &longest_axis,
            )
        });

        let mid = start + (diff as f64 / 2.0) as usize;

        let left_children = Self::split_into_nodes(nodes, objects, start, mid);
        let right_children = Self::split_into_nodes(nodes, objects, mid, end);

        let left_node = Self::new_node(nodes, left_children.0, left_children.1);
        nodes.push(BvhValue::Node(left_node));
        let right_node = Self::new_node(nodes, right_children.0, right_children.1);
        nodes.push(BvhValue::Node(right_node));
        let len = nodes.len();
        (len - 2, len - 1)
    }

    fn new_node(nodes: &[BvhValue], left_id: usize, right_id: usize) -> BvhNode {
        let bb_left = nodes[left_id].bounding_box();
        let bb_right = nodes[right_id].bounding_box();
        BvhNode {
            left_id,
            right_id,
            bounding_box: AxisAlignedBoundingBox::merge(bb_left, bb_right),
        }
    }
}

impl HittableObject for BvhTree {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord> {
        self.hit_node(self.root(), ray, t_range)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        match self.root() {
            BvhValue::Node(bvh_node) => bvh_node.bounding_box(),
            BvhValue::Leaf(bvh_leaf) => bvh_leaf.bounding_box(),
        }
    }
}

impl From<Vec<HittableObjectType>> for BvhTree {
    fn from(mut value: Vec<HittableObjectType>) -> Self {
        assert!(!value.is_empty());
        let mut nodes = Vec::new();
        nodes.push(BvhValue::Node(BvhNode::empty()));
        let end = value.len();
        let (left, right) = Self::split_into_nodes(&mut nodes, &mut value, 0, end);
        let root = Self::new_node(&nodes, left, right);
        nodes[0] = BvhValue::Node(root);
        Self { nodes }
    }
}

pub enum BvhValue {
    Node(BvhNode),
    Leaf(BvhLeaf),
}

impl BvhValue {
    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        match self {
            BvhValue::Node(node) => node.bounding_box(),
            BvhValue::Leaf(leaf) => leaf.bounding_box(),
        }
    }
}

/// For efficency reasons we don't allow "null pointers" in our tree, so there sometimes might be duplicated
/// elements in it - in our case it's fine.
pub struct BvhNode {
    left_id: usize,
    right_id: usize,
    bounding_box: AxisAlignedBoundingBox,
}

impl BvhNode {
    fn empty() -> Self {
        BvhNode {
            left_id: usize::MAX,
            right_id: usize::MAX,
            bounding_box: AxisAlignedBoundingBox::empty(),
        }
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}

pub struct BvhLeaf {
    object: HittableObjectType,
}

impl HittableObject for BvhLeaf {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord> {
        self.object.hit(ray, t_range)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        self.object.bounding_box()
    }
}
