extern crate nalgebra_glm as glm;

use crate::component::GeometryContainer;
use crate::component::TransformComponent;
use crate::IApi;

#[allow(dead_code)]
#[repr(C)]
struct ViewData {
    model_matrix: glm::Mat4x4,
}

pub struct Scene<TApi: IApi> {
    transforms: Vec<TransformComponent>,
    geometry_container: GeometryContainer<TApi>,
}

impl<TApi: IApi> Scene<TApi> {
    pub fn new(device: &mut TApi::Device) -> Self {
        let transform = TransformComponent::new();

        Self {
            transforms: vec![transform],
            geometry_container: GeometryContainer::new(device),
        }
    }

    pub fn get_geometry_container(&self) -> &GeometryContainer<TApi> {
        &self.geometry_container
    }

    pub fn get_geometry_container_mut(&mut self) -> &mut GeometryContainer<TApi> {
        &mut self.geometry_container
    }

    pub fn transforms(&self) -> &[TransformComponent] {
        &self.transforms
    }

    pub fn transforms_mut(&mut self) -> &mut [TransformComponent] {
        &mut self.transforms
    }
}

pub struct SceneUpdater<TApi: IApi> {
    _marker: std::marker::PhantomData<TApi>,
}

impl<TApi: IApi> SceneUpdater<TApi> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn update(&self, scene: &mut Scene<TApi>) {
        // トランスフォームの更新
        for transform in scene.transforms_mut() {
            transform.update();
        }

        // ジオメトリの更新
        // for constant_buffer in scene.get_geometry_container_mut().get_constant_buffers() {
        //     constant_buffer.map_mut(|data: &mut ViewData| {
        //         data.model_matrix = glm::Mat4x4::identity();
        //     });
        // }
    }
}
