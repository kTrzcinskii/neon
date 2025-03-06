pub mod scene_generator;

use crate::{bvh::BvhTree, material::MaterialType};

pub struct Scene {
    materials: Vec<MaterialType>,
    bvh: BvhTree,
}

impl Scene {
    pub fn new(materials: Vec<MaterialType>, bvh: BvhTree) -> Self {
        Scene { materials, bvh }
    }

    pub fn bvh(&self) -> &BvhTree {
        &self.bvh
    }

    pub fn material_by_id(&self, id: usize) -> Option<&MaterialType> {
        if id >= self.materials.len() {
            return None;
        }
        Some(&self.materials[id])
    }
}
