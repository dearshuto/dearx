use sjgfx::api::IApi;

use crate::component::{GeometryComponent, TransformComponent};

pub struct Scene<TApi: IApi> {
    transforms: Vec<TransformComponent>,
    geometries: Vec<GeometryComponent<TApi>>,
}

impl<TApi: IApi> Scene<TApi> {
    pub fn new(device: &TApi::Device) -> Self {
        let transform = TransformComponent::new();
        let geometry = GeometryComponent::<TApi>::new(device, 0 /*transform_index*/);

        Self {
            transforms: vec![transform],
            geometries: vec![geometry],
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
            let transform = &mut self.transforms[index as usize];

            geometry.update(transform);
        }
    }

    pub fn transforms(&self) -> &[TransformComponent] {
        &self.transforms
    }

    pub fn geometries(&self) -> &[GeometryComponent<TApi>] {
        &self.geometries
    }
}
