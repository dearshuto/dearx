pub mod domain;

// pub mod experimental;

#[cfg(not(target_arch = "wasm32"))]
pub mod component;
#[cfg(not(target_arch = "wasm32"))]
mod renderer;
#[cfg(not(target_arch = "wasm32"))]
mod scene;

#[cfg(feature = "viewer")]
pub mod viewer;

#[cfg(not(target_arch = "wasm32"))]
pub use renderer::Renderer;
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