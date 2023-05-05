#[allow(clippy::module_inception)]
mod renderer;

pub mod wgpu;

pub use renderer::*;

pub struct SceneObject<TBuffer, TPipeline> {
    pub vertex_buffers: Vec<TBuffer>,
    pub pipelines: Vec<TPipeline>,
}
