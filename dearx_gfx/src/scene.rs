use crate::component::{GeometryComponent, TransformComponent};

pub struct Scene {
    transforms: Vec<TransformComponent>,
    geometries: Vec<GeometryComponent>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            transforms: Vec::new(),
            geometries: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        // トランスフォームの更新
        for transform in &mut self.transforms {
            transform.update();
        }

        // ジオメトリの更新
        for geometry in &mut self.geometries {
            let index = geometry.get_transform_index();
            let transform = &self.transforms[index as usize];

            geometry.update(transform);
        }
    }

    pub fn transforms(&self) -> &[TransformComponent] {
        &self.transforms
    }

    pub fn geometries(&self) -> &[GeometryComponent] {
        &self.geometries
    }
}
