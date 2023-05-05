pub mod api;
pub mod domain;
pub mod experimental;
pub mod serializer;

#[cfg(not(target_arch = "wasm32"))]
pub mod component;
mod renderer;
#[cfg(not(target_arch = "wasm32"))]
mod scene;

pub use api::{IApi, IBuffer, IDevice, ITexture};

pub use renderer::wgpu;
#[cfg(not(target_arch = "wasm32"))]
pub use renderer::Renderer;
pub use renderer::{DrawCommandInfo, ICommandBuffer, IDrawInfo, IGraphicsObjectId, IScene};

#[cfg(not(target_arch = "wasm32"))]
pub use scene::{Scene, SceneUpdater};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
