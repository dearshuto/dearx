pub mod component;
pub mod domain;
mod renderer;
mod scene;
pub use renderer::Renderer;
pub use scene::Scene;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
