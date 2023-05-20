use crate::IEditBuffer;

impl IEditBuffer for wgpu::Buffer {
    fn write(&mut self, _data: &crate::CameraData) {
        todo!()
    }
}
