use crate::renderer::SceneObject;

pub trait IFactory {
    type TBuffer;
    type TRenderPipeline;

    fn create_buffer(&self, descriptor: &CreateBufferDescriptor) -> Self::TBuffer;

    fn create_render_pipeline(
        &self,
        descriptor: &CreateRenderPipelineDescriptor,
    ) -> Self::TRenderPipeline;
}

pub struct CreateBufferDescriptor<'a> {
    pub data: &'a [u8],
}

pub struct CreateRenderPipelineDescriptor<'a> {
    pub vertex_shader: &'a [u8],
    pub pixel_shader: Option<&'a [u8]>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn deserialize<TFactory: IFactory>(
    _data: &[u8],
    factory: &mut TFactory,
) -> SceneObject<TFactory::TBuffer, TFactory::TRenderPipeline> {
    // let mut stream_reader = usd_rs::StreamReader::new(data);
    // let _reader = usd_rs::AsciiReader::new(&mut stream_reader);

    // シェーダー
    let vertex_shader_source = include_str!("../../resources/shaders/triangle.vs");
    let pixel_shader_source = include_str!("../../resources/shaders/triangle.fs");
    let mut compiler = sjgfx_util::ShaderCompiler::new();
    let vertex_shader_binary =
        compiler.create_binary(vertex_shader_source, sjgfx_util::ShaderStage::Vertex);
    let pixel_shader_binary =
        compiler.create_binary(pixel_shader_source, sjgfx_util::ShaderStage::Pixel);
    let pipeline = factory.create_render_pipeline(&CreateRenderPipelineDescriptor {
        vertex_shader: &vertex_shader_binary,
        pixel_shader: Some(&pixel_shader_binary),
    });

    let vertex_buffer_data0 =
        bytemuck::cast_slice(&[-0.40f32, -0.25, 0.0, 0.10, -0.25, 0.0, -0.15, 0.25, 0.0]);
    let vertex_buffer_data1 =
        bytemuck::cast_slice(&[-0.10f32, 0.25, 0.0, 0.40, 0.25, 0.0, 0.15, -0.25, 0.0]);

    let vertex_buffer0 = factory.create_buffer(&CreateBufferDescriptor {
        data: vertex_buffer_data0,
    });
    let vertex_buffer1 = factory.create_buffer(&CreateBufferDescriptor {
        data: vertex_buffer_data1,
    });

    SceneObject {
        vertex_buffers: vec![vertex_buffer0, vertex_buffer1],
        pipelines: vec![pipeline],
    }
}
