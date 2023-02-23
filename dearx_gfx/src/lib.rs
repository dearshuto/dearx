pub mod component;
pub mod domain;
mod renderer;
mod scene;

#[cfg(feature = "viewer")]
pub mod viewer;

pub use renderer::Renderer;
pub use scene::{Scene, SceneUpdater};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
