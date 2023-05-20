mod container;
#[allow(clippy::module_inception)]
mod renderer;
mod scene;

pub mod wgpu;

pub use container::{TableContainer, VectorContainer};
pub use renderer::*;
pub use scene::Scene;

pub struct SceneObject<TBuffer, TPipeline, TDescriptorPool> {
    pub vertex_buffers: Vec<TBuffer>,
    pub constant_buffers: Vec<TBuffer>,
    pub pipelines: Vec<TPipeline>,
    pub descriptor_pool: Vec<TDescriptorPool>,
}
