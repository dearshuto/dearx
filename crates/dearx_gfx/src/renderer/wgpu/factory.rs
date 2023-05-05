use wgpu::{util::DeviceExt, Device};

use crate::serializer::{CreateBufferDescriptor, IFactory};

pub struct Factory<'a> {
    device: &'a wgpu::Device,
}

impl<'a> Factory<'a> {
    pub fn new(device: &'a Device) -> Self {
        Self { device }
    }
}

impl<'a> IFactory for Factory<'a> {
    type TBuffer = wgpu::Buffer;

    fn create_buffer(&self, descriptor: &CreateBufferDescriptor) -> Self::TBuffer {
        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: descriptor.data,
                usage: wgpu::BufferUsages::VERTEX,
            });

        buffer
    }
}
