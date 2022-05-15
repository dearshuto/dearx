use sje_generator_macro::Immutable;
use std::sync::Arc;

pub mod components;
mod float3;
mod game_object;
pub use float3::Float3;
pub use game_object::GameObject;

#[derive(Immutable)]
pub struct TestData {
    pub value: i32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
