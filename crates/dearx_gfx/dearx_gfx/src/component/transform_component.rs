use crate::domain::Float3;

pub struct TransformComponent {
    pub translation: Float3,
    pub rotation: Float3,
    pub scale: Float3,
}

impl TransformComponent {
    pub fn new() -> Self {
        Self {
            translation: Float3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Float3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Float3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }

    pub fn update(&mut self) {}
}
