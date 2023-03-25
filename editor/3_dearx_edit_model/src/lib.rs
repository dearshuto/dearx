mod contents;

use uuid::Uuid;

mod dearx_project;
pub use dearx_project::DearxProject;
pub mod components;
mod float3;
mod game_object;
pub use contents::{Model, ModelContent};
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
