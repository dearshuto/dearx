use uuid::Uuid;

mod geometry_component;
mod static_mesh_component;
mod transform_component;
pub use geometry_component::GeometryComponent;
pub use static_mesh_component::StaticMeshComponent;
pub use transform_component::TransformComponent;

#[derive(Debug, Clone, Copy)]
pub struct ComponentId {
    #[allow(dead_code)]
    id: Uuid,
}

impl ComponentId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}
