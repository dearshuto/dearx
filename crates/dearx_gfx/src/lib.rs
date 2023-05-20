pub mod api;
pub mod domain;
pub mod experimental;
pub mod serializer;

#[cfg(not(target_arch = "wasm32"))]
pub mod component;
mod renderer;

pub use api::{IApi, IBuffer, IDevice, ITexture};

pub use renderer::wgpu;
pub use renderer::{
    DrawCommandInfo, ICommandBuffer, IDrawInfo, IGraphicsObjectId, IScene, SceneObject,
};
#[cfg(not(target_arch = "wasm32"))]
pub use renderer::{Renderer, Scene};

#[repr(C)]
#[derive(Clone, Copy)]
struct ModelData {
    modelmatrix: [f32; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ViewData {
    projection_view_matrix: [f32; 16],
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
