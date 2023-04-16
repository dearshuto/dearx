extern crate nalgebra_glm as glm;

use crate::IApi;

#[allow(dead_code)]
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
    pub fn new(_device: &mut TApi::Device) -> Self {
        Self {
            vertex_buffers: vec![],
            index_buffers: vec![],
            constant_buffers: vec![],
            index_counts: vec![],
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
