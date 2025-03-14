pub mod scene_generator;

use rgb::Rgb;
use typed_builder::TypedBuilder;

use crate::{
    core::bvh::BvhTree, core::camera::Camera, core::rendered_image::RenderedImage,
    material::MaterialType,
};

pub struct Scene {
    camera: Camera,
    content: SceneContent,
    options: SceneOptions,
}

impl Scene {
    pub fn new(content: SceneContent, camera: Camera, options: SceneOptions) -> Self {
        Self {
            content,
            camera,
            options,
        }
    }

    pub fn render(&self) -> RenderedImage {
        self.camera.render(&self.content, &self.options)
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

// TODO: for background use texture instead of color
#[derive(TypedBuilder)]
pub struct SceneOptions {
    background: Rgb<f64>,
}

impl SceneOptions {
    pub fn background(&self) -> &Rgb<f64> {
        &self.background
    }
}

impl Default for SceneOptions {
    fn default() -> Self {
        Self {
            background: Rgb::new(0.7, 0.8, 1.0),
        }
    }
}
