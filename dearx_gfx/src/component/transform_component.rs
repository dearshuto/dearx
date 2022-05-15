use crate::domain::Float3;

pub struct TransformComponent {
    pub translation: Float3,
    pub rotation: Float3,
    pub scale: Float3,
}

impl TransformComponent {
    pub fn update(&mut self) {}
}
