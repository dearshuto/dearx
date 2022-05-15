use im::HashMap;
use sje_generator_macro::Immutable;
use std::sync::Arc;

use crate::components::{ComponentId, StaticMeshComponent, TransformComponent};

#[derive(Immutable)]
pub struct GameObject {
    pub transform_components: Arc<HashMap<ComponentId, TransformComponent>>,
    pub static_mesh_components: Arc<HashMap<ComponentId, StaticMeshComponent>>,
}

impl GameObject {
    pub fn new() -> Self {
        Self {
            transform_components: Arc::new(HashMap::new()),
            static_mesh_components: Arc::new(HashMap::new()),
        }
    }

    pub fn test(&self, value: Arc<HashMap<ComponentId, TransformComponent>>) -> Arc<Self> {
        let instance = Self {
            transform_components: value,
            static_mesh_components: self.static_mesh_components.clone(),
        };

        Arc::new(instance)
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{ComponentId, TransformComponent};
    use crate::GameObject;
    use im::HashMap;
    use std::sync::Arc;

    #[test]
    fn new() {
        let game_object = GameObject::new();
        let instance = HashMap::<ComponentId, TransformComponent>::new();

        let _ = game_object.test(Arc::new(instance));
    }
}
