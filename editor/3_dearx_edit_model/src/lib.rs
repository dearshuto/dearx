use dearx_macro::Immutable;
use dearx_workspace::DocumentId;
use im::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub mod components;
mod float3;
mod game_object;
pub use float3::Float3;
pub use game_object::GameObject;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
pub struct DearxProject {
    pub game_object: Arc<HashMap<GameObjectId, GameObject>>,
    pub selections: Arc<im::Vector<DocumentId>>,
}

impl DearxProject {
    pub fn new() -> Self {
        Self {
            game_object: Default::default(),
            selections: Default::default(),
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