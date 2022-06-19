extern crate nalgebra_glm as glm;

pub struct CameraData {
    camera_position: glm::Vec3,
    camera_direction: glm::Vec3,
}

impl CameraData {
    pub fn calculate_view_matrix(&self) -> glm::Mat4x4 {
        let at = self.camera_position + self.camera_direction;
        let up = glm::Vec3::new(0.0, 1.0, 0.0);
        let view_matrix = glm::look_at(&self.camera_position, &at, &up);
        view_matrix
    }
}

impl Default for CameraData {
    fn default() -> Self {
        Self {
            camera_position: glm::Vec3::new(0.0, 0.0, 10.0),
            camera_direction: glm::Vec3::new(0.0, 0.0, -1.0),
        }
    }
}

pub struct Manipulator;

impl Manipulator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn manipulate(&self, _camera_data: &mut CameraData) {
        Default::default()
    }
}
