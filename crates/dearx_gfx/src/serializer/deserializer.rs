use usd_rs::{LoadState, Path};

use crate::{renderer::SceneObject, ModelData, ViewData};

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
        descriptor: &CreateDescriptorPoolDescriptor<Self::TBuffer>,
    ) -> Self::TDescriptorPool;
}

pub struct CreateBufferDescriptor<'a> {
    pub data: &'a [u8],
    pub gpu_access: sjgfx_interface::GpuAccess,
}

pub struct CreateRenderPipelineDescriptor<'a> {
    pub vertex_shader: &'a [u8],
    pub pixel_shader: Option<&'a [u8]>,
    pub vertex_buffer_offsets: Vec<isize>,
    pub texture_format: Option<sjgfx_interface::ImageFormat>,
}

pub struct CreateDescriptorPoolDescriptor<'a, TBuffer> {
    pub vertex_shader: &'a [u8],
    pub pixel_shader: &'a [u8],
    pub constant_buffers: &'a [&'a TBuffer],
}

pub fn deserialize<TFactory: IFactory>(
    _data: &[u8],
    factory: &mut TFactory,
) -> SceneObject<TFactory::TBuffer, TFactory::TRenderPipeline, TFactory::TDescriptorPool> {
    // 2D 描画シェーダー
    let (pipeline, vertex_shader_binary, pixel_shader_binary) = create_render_pipeline(
        factory,
        include_str!("../../resources/shaders/triangle.vs"),
        include_str!("../../resources/shaders/triangle.fs"),
        vec![(std::mem::size_of::<f32>() * 3) as isize],
    );

    // 3D 描画シェーダー
    let (pipeline_3d, vertex_shader_binary_3d, pixel_shader_binary_3d) = create_render_pipeline(
        factory,
        include_str!("../../resources/shaders/basic.vs"),
        include_str!("../../resources/shaders/basic.fs"),
        vec![(std::mem::size_of::<f32>() * 6) as isize],
    );

    let vertex_buffer_data0 =
        bytemuck::cast_slice(&[-0.40f32, -0.25, 0.0, -0.15, 0.25, 0.0, 0.10, -0.25, 0.0]);
    let vertex_buffer_data1 =
        bytemuck::cast_slice(&[-0.10f32, 0.25, 0.0, 0.40, 0.25, 0.0, 0.15, -0.25, 0.0]);
    let vertex_buffer_3d_data_vec = load_3d();
    let vertex_buffer_3d_data = bytemuck::cast_slice(&vertex_buffer_3d_data_vec);

    let vertex_buffer0 = factory.create_buffer(&CreateBufferDescriptor {
        data: vertex_buffer_data0,
        gpu_access: sjgfx_interface::GpuAccess::VERTEX_BUFFER,
    });
    let vertex_buffer1 = factory.create_buffer(&CreateBufferDescriptor {
        data: vertex_buffer_data1,
        gpu_access: sjgfx_interface::GpuAccess::VERTEX_BUFFER,
    });
    let vertex_buffer_3d = factory.create_buffer(&CreateBufferDescriptor {
        data: vertex_buffer_3d_data,
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
    let view_matrix = nalgebra_glm::look_at_lh(
        &nalgebra_glm::Vec3::new(5.0, 3.0, 5.0),
        &nalgebra_glm::Vec3::zeros(),
        &nalgebra_glm::Vec3::new(0.0, 1.0, 0.0),
    );
    let projection_matrix =
        nalgebra_glm::perspective_fov_lh(30.0_f32.to_radians(), 640.0, 480.0, 0.1, 100.0);
    let pv = (projection_matrix * view_matrix).transpose();
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
        constant_buffers: &[],
    });
    let bind_group_3d = factory.create_descriptor_pool(&CreateDescriptorPoolDescriptor {
        vertex_shader: &vertex_shader_binary_3d,
        pixel_shader: &pixel_shader_binary_3d,
        constant_buffers: &[&model_data_buffer, &view_buffer, &model_data_buffer], // TODO: ここ適当なので直す
    });

    SceneObject {
        vertex_buffers: vec![vertex_buffer0, vertex_buffer1, vertex_buffer_3d],
        constant_buffers: vec![model_data_buffer, view_buffer],
        pipelines: vec![pipeline, pipeline_3d],
        descriptor_pool: vec![bind_group, bind_group_3d],
    }
}

fn create_render_pipeline<TFactory>(
    factory: &mut TFactory,
    vertex_shader_source: &str,
    pixel_shader_source: &str,
    vertex_buffer_offsets: Vec<isize>,
) -> (TFactory::TRenderPipeline, Vec<u8>, Vec<u8>)
where
    TFactory: IFactory,
{
    let mut compiler = sjgfx_util::ShaderCompiler::new();
    let vertex_shader_binary =
        compiler.create_binary(vertex_shader_source, sjgfx_util::ShaderStage::Vertex);
    let pixel_shader_binary =
        compiler.create_binary(pixel_shader_source, sjgfx_util::ShaderStage::Pixel);
    let pipeline = factory.create_render_pipeline(&CreateRenderPipelineDescriptor {
        vertex_shader: &vertex_shader_binary,
        pixel_shader: Some(&pixel_shader_binary),
        vertex_buffer_offsets,
        texture_format: None,
    });
    (pipeline, vertex_shader_binary, pixel_shader_binary)
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

fn load_3d() -> Vec<f32> {
    let data = include_str!("../../resources/models/cube.usda");
    let mut stream_reader = usd_rs::StreamReader::new(data.as_bytes());
    let mut reader = usd_rs::AsciiReader::new(&mut stream_reader);
    reader.read(LoadState::TopLevel);
    reader.reconstruct_stage();

    let default_data = vec![
        -0.25f32, -0.25, 0.0, // x
        0.0, 0.0, -1.0, // nx
        0.25, -0.25, 0.0, // y
        0.0, 0.0, -1.0, // ny
        0.0, 0.25, 0.0, // z
        0.0, 0.0, -1.0, // nz
    ];
    let Some(stage) = reader.try_get_stage() else {
        return default_data;
    };

    let path = Path::new("/Cube", "");
    let Some(prim) = stage.find_prim_at_path(&path) else {
        return default_data;
    };

    let Some(child) = prim.try_get_child(0) else {
        return default_data;
     };

    let Some(geom_mesh) = child.as_gemo_mesh() else {
         return default_data;
     };

    // 頂点は 8 つ、Index を走査して頂点バッファーを構築する必要がある
    let mut data = Vec::default();
    for index in 0..(geom_mesh.get_index_count() / 3) {
        for local_index in 0..3 {
            let actual_index = geom_mesh.get_index(3 * index + local_index);
            let point = geom_mesh.get_point(actual_index);
            let normal = geom_mesh.get_normal(actual_index);
            data.push(point.0);
            data.push(point.1);
            data.push(point.2);
            data.push(normal.0);
            data.push(normal.1);
            data.push(normal.2);
        }
    }
    data
}
