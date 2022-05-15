use super::TransformComponent;

pub struct GeometryComponent;

impl GeometryComponent {
    pub fn update(&mut self, _transform: &TransformComponent) {}

    pub fn get_transform_index(&self) -> u32 {
        todo!()
    }
}
