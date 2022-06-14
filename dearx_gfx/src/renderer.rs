use sjgfx::{api::IApi, TShaderBuilder, TCommandBufferBuilder};
use sjgfx_interface::ICommandBuffer;

use crate::Scene;

pub struct Renderer<TApi: IApi> {
    basic_shader: TApi::Shader,
    command_buffer: TApi::CommandBuffer,
    _marker: std::marker::PhantomData<TApi>,
}

impl<TApi: IApi> Renderer<TApi> {
    pub fn new(device: &TApi::Device) -> Self {
        let shader = TShaderBuilder::<TApi>::new()
            .set_vertex_shader_binary(include_bytes!("../outputs/resources/shaders/basic.vs.spv"))
            .set_pixel_shader_binary(include_bytes!("../outputs/resources/shaders/basic.fs.spv"))
            .build(device);
        let command_buffer = TCommandBufferBuilder::<TApi>::new().build(device);

        Self {
            basic_shader: shader,
            command_buffer,
            _marker: std::marker::PhantomData
        }
    }

    pub fn make_command(&mut self, scene: &Scene) {
        for _ in scene.geometries() {
            self.command_buffer.begin();
            self.command_buffer.set_shader(&self.basic_shader);
            self.command_buffer.end();
        }
    }

    pub fn get_command_buffers(&self) -> &[TApi::CommandBuffer] {
        &[]
    }
}
