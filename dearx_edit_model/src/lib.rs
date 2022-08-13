use im::HashMap;
use sje_generator_macro::Immutable;
use uuid::Uuid;
use std::sync::Arc;

pub mod components;
mod float3;
mod game_object;
pub use float3::Float3;
pub use game_object::GameObject;

#[derive(Debug, Clone, Copy)]
pub struct GameObjectId {
    #[allow(dead_code)]
    id: Uuid,
}

impl GameObjectId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

#[derive(Immutable)]
pub struct DearxProject
{
    pub game_object: Arc<HashMap<GameObjectId, GameObject>>,
    pub value:f64,
}

impl DearxProject {
    pub fn new() -> Self {
        Self{
            game_object: Default::default(),
            value: 0.0
        }
    }
}

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
