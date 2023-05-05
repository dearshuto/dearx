#[allow(clippy::module_inception)]
mod renderer;

pub mod wgpu;

pub use renderer::*;

pub struct SceneObject<TBuffer, TPipeline, TDescriptorPool> {
    pub vertex_buffers: Vec<TBuffer>,
    pub constant_buffers: Vec<TBuffer>,
    pub pipelines: Vec<TPipeline>,
    pub descriptor_pool: Vec<TDescriptorPool>,
}
