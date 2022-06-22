extern crate nalgebra_glm as glm;

#[derive(Clone)]
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

pub struct Manipulator {
    init_camera_data: CameraData,
    init_mouse_position: (f64, f64),
    current_mouse_event: Option<sjvi::MouseEvent>,
}

impl Manipulator {
    pub fn try_new(mouse_event: &sjvi::MouseEvent, init_camera_data: &CameraData) -> Option<Self> {
        if let sjvi::MouseEvent::Pressed(x, y, _) = mouse_event {
            Some(Self {
                init_camera_data: init_camera_data.clone(),
                init_mouse_position: (*x, *y),
                current_mouse_event: None,
            })
        } else {
            None
        }
    }

    pub fn push(mut self, mouse_event: &sjvi::MouseEvent) -> Option<Manipulator> {
        if let sjvi::MouseEvent::Moved(x, y) = mouse_event {
            self.current_mouse_event = Some(sjvi::MouseEvent::Moved(*x, *y));
            Some(self)
        } else {
            None
        }
    }

    pub fn try_calculate(&self) -> Option<CameraData> {
        if let Some(sjvi::MouseEvent::Moved(x, _y)) = self.current_mouse_event {
            let angle = (self.init_mouse_position.0 - x) / 100.0; // 適当
            let init_camera_position = glm::Vec3::new(
                self.init_camera_data.camera_position.x,
                self.init_camera_data.camera_position.y,
                self.init_camera_data.camera_position.z,
            );
            let camera_position = glm::rotate_y_vec3(&init_camera_position, angle as f32);
            let camera_direction = glm::Vec3::zeros() - camera_position;
            Some(CameraData {
                camera_position,
                camera_direction,
            })
        } else {
            None
        }
    }

    pub fn manipulate(&self, _camera_data: &mut CameraData) {
        Default::default()
    }
}
