extern crate nalgebra_glm as glm;

use sjgfx::{api::IApi, TBufferBuilder};

#[repr(C)]
struct ViewData {
    model_matrix: glm::Mat4x4,
}

pub struct GeometryContainer<TApi: IApi> {
    vertex_buffers: Vec<TApi::Buffer>,
    index_buffers: Vec<TApi::Buffer>,
    constant_buffers: Vec<TApi::Buffer>,
    index_counts: Vec<i32>,
}

impl<TApi: IApi> GeometryContainer<TApi> {
    pub fn new(device: &mut TApi::Device) -> Self {
        let obj_data =
            sjgfx_util::load_obj(device, include_str!("../../../resources/models/cube.obj"));
        let constant_buffer = TBufferBuilder::<TApi>::new()
            .enable_constant_buffer()
            .with_size(std::mem::size_of::<ViewData>())
            .build(device);

        Self {
            vertex_buffers: vec![obj_data.vertex_buffer],
            index_buffers: vec![obj_data.index_buffer],
            constant_buffers: vec![constant_buffer],
            index_counts: vec![obj_data.index_count],
        }
    }

    pub fn get_vertex_buffers(&self) -> &[TApi::Buffer] {
        &self.vertex_buffers
    }

    pub fn get_index_buffers(&self) -> &[TApi::Buffer] {
        &self.index_buffers
    }

    pub fn get_constant_buffers(&self) -> &[TApi::Buffer] {
        &self.constant_buffers
    }

    pub fn get_index_counts(&self) -> &[i32] {
        &self.index_counts
    }
}
