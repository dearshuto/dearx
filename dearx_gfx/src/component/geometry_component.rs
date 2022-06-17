extern crate nalgebra_glm as glm;

use sjgfx::{api::IApi, TBufferBuilder};
use sjgfx_interface::IBuffer;

use super::TransformComponent;

#[repr(C)]
struct ViewData {
    model_matrix: glm::Mat4x4,
}

pub struct GeometryComponent<TApi: IApi> {
    vertex_buffer: TApi::Buffer,
    index_buffer: TApi::Buffer,
    constant_buffer: TApi::Buffer,
    transform_index: u32,
    index_count: i32,
}

impl<TApi: IApi> GeometryComponent<TApi> {
    pub fn new(device: &TApi::Device, transform_index: u32) -> Self {
        let obj_data =
            sjgfx_util::load_obj(device, include_str!("../../../resources/models/cube.obj"));
        let constant_buffer = TBufferBuilder::<TApi>::new()
            .enable_constant_buffer()
            .with_size(std::mem::size_of::<ViewData>())
            .build(device);

        Self {
            vertex_buffer: obj_data.vertex_buffer,
            index_buffer: obj_data.index_buffer,
            constant_buffer,
            transform_index,
            index_count: obj_data.index_count,
        }
    }

    pub fn update(&mut self, transform: &mut TransformComponent) {
        self.constant_buffer.map_mut(|data: &mut ViewData| {
            let translation = glm::vec3(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            );
            let translation_matrix = glm::translation(&translation);
            data.model_matrix = translation_matrix;
        });
    }

    pub fn get_transform_index(&self) -> u32 {
        self.transform_index
    }

    pub fn get_vertex_buffer(&self) -> &TApi::Buffer {
        &self.vertex_buffer
    }

    pub fn get_index_buffer(&self) -> &TApi::Buffer {
        &self.index_buffer
    }

    pub fn get_index_count(&self) -> i32 {
        self.index_count
    }

    pub fn get_model_data(&self) -> &TApi::Buffer {
        &self.constant_buffer
    }
}
