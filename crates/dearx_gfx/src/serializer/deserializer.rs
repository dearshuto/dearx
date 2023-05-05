use crate::{renderer::SceneObject, ModelData, ViewData};
use nalgebra_glm;

pub trait IFactory {
    type TBuffer;
    type TRenderPipeline;
    type TDescriptorPool;

    fn create_buffer(&self, descriptor: &CreateBufferDescriptor) -> Self::TBuffer;

    fn create_render_pipeline(
        &self,
        descriptor: &CreateRenderPipelineDescriptor,
    ) -> Self::TRenderPipeline;

    fn create_descriptor_pool(
        &self,
        descriptor: &CreateDescriptorPoolDescriptor,
    ) -> Self::TDescriptorPool;
}

pub struct CreateBufferDescriptor<'a> {
    pub data: &'a [u8],
    pub gpu_access: sjgfx_interface::GpuAccess,
}

pub struct CreateRenderPipelineDescriptor<'a> {
    pub vertex_shader: &'a [u8],
    pub pixel_shader: Option<&'a [u8]>,
    pub texture_format: Option<sjgfx_interface::ImageFormat>,
}

pub struct CreateDescriptorPoolDescriptor<'a> {
    pub vertex_shader: &'a [u8],
    pub pixel_shader: &'a [u8],
}

pub fn deserialize<TFactory: IFactory>(
    _data: &[u8],
    factory: &mut TFactory,
) -> SceneObject<TFactory::TBuffer, TFactory::TRenderPipeline, TFactory::TDescriptorPool> {
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
        texture_format: None,
    });

    let vertex_buffer_data0 =
        bytemuck::cast_slice(&[-0.40f32, -0.25, 0.0, 0.10, -0.25, 0.0, -0.15, 0.25, 0.0]);
    let vertex_buffer_data1 =
        bytemuck::cast_slice(&[-0.10f32, 0.25, 0.0, 0.40, 0.25, 0.0, 0.15, -0.25, 0.0]);

    let vertex_buffer0 = factory.create_buffer(&CreateBufferDescriptor {
        data: vertex_buffer_data0,
        gpu_access: sjgfx_interface::GpuAccess::VERTEX_BUFFER,
    });
    let vertex_buffer1 = factory.create_buffer(&CreateBufferDescriptor {
        data: vertex_buffer_data1,
        gpu_access: sjgfx_interface::GpuAccess::VERTEX_BUFFER,
    });

    // モデルの定数バッファ
    let model_data_buffer = factory.create_buffer(&CreateBufferDescriptor {
        data: unsafe {
            any_as_u8_slice(&ModelData {
                modelmatrix: convert(&nalgebra_glm::Mat4x4::identity()),
            })
        },
        gpu_access: sjgfx_interface::GpuAccess::CONSTANT_BUFFER,
    });

    // PV の定数バッファ
    let pv = nalgebra_glm::perspective_fov(0.14, 640.0, 480.0, 0.1, 100.0);
    let view_buffer = factory.create_buffer(&CreateBufferDescriptor {
        data: unsafe {
            any_as_u8_slice(&ViewData {
                projection_view_matrix: convert(&pv),
            })
        },
        gpu_access: sjgfx_interface::GpuAccess::CONSTANT_BUFFER,
    });

    // デスクリプター設定
    let bind_group = factory.create_descriptor_pool(&CreateDescriptorPoolDescriptor {
        vertex_shader: &vertex_shader_binary,
        pixel_shader: &pixel_shader_binary,
    });

    SceneObject {
        vertex_buffers: vec![vertex_buffer0, vertex_buffer1],
        constant_buffers: vec![model_data_buffer, view_buffer],
        pipelines: vec![pipeline],
        descriptor_pool: vec![bind_group],
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

fn convert(matrix: &nalgebra_glm::Mat4x4) -> [f32; 16] {
    let data = matrix.as_slice();
    [
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9],
        data[10], data[11], data[12], data[13], data[14], data[15],
    ]
}
