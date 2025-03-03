use crate::{material::MaterialType, object::HittableObjectType};

pub struct Scene {
    materials: Vec<MaterialType>,
    world: HittableObjectType,
}

impl Scene {
    pub fn new(materials: Vec<MaterialType>, world: HittableObjectType) -> Self {
        Scene { materials, world }
    }

    pub fn world(&self) -> &HittableObjectType {
        &self.world
    }

    pub fn material_by_id(&self, id: usize) -> Option<&MaterialType> {
        if id >= self.materials.len() {
            return None;
        }
        Some(&self.materials[id])
    }
}
