pub mod scene_generator;

use crate::{
    core::bvh::BvhTree, core::camera::Camera, core::rendered_image::RenderedImage,
    material::MaterialType,
};

pub struct Scene {
    content: SceneContent,
    camera: Camera,
}

impl Scene {
    pub fn new(content: SceneContent, camera: Camera) -> Self {
        Self { content, camera }
    }

    pub fn render(&self) -> RenderedImage {
        self.camera.render(&self.content)
    }
}

pub struct SceneContent {
    materials: Vec<MaterialType>,
    bvh: BvhTree,
}

impl SceneContent {
    pub fn new(materials: Vec<MaterialType>, bvh: BvhTree) -> Self {
        SceneContent { materials, bvh }
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
