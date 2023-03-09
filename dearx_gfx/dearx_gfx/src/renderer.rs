extern crate nalgebra_glm as glm;

use sjgfx::{
    api::IApi, TBufferBuilder, TCommandBufferBuilder, TShaderBuilder, TTextureBuilder,
    TVertexStateBuilder,
};
use sjgfx_interface::{
    AttributeFormat, DepthStencilStateInfo, IBuffer, ICommandBuffer, IDepthStencilView,
    ImageFormat, IndexFormat, PrimitiveTopology, VertexAttributeStateInfo, VertexBufferStateInfo,
};

use crate::Scene;

#[repr(C)]
struct LightData {
    light_direction: glm::Vec4,
    color: glm::Vec4,
}

#[repr(C)]
struct ViewData {
    projection_view_matrix: glm::Mat4x4,
}

pub struct Renderer<TApi: IApi> {
    #[allow(dead_code)]
    depth_buffer: TApi::Texture,
    depth_stencil_view: TApi::DepthStencilView,
    basic_shader: TApi::Shader,
    view_constant_buffer: TApi::Buffer,
    light_constant_buffer: TApi::Buffer,
    vertex_state: TApi::VertexState,
    command_buffers: [TApi::CommandBuffer; 1],
}

impl<TApi: IApi> Renderer<TApi> {
    pub fn new(device: &mut TApi::Device) -> Self {
        // シェーダ
        let shader = TShaderBuilder::<TApi>::new()
            .set_vertex_shader_binary(include_bytes!("../outputs/resources/shaders/basic.vs.spv"))
            .set_pixel_shader_binary(include_bytes!("../outputs/resources/shaders/basic.fs.spv"))
            .build(device);

        // 定数バッファ
        let constnat_buffer = TBufferBuilder::<TApi>::new()
            .enable_constant_buffer()
            .with_size(std::mem::size_of::<ViewData>())
            .build(device);
        constnat_buffer.map_mut(|data: &mut ViewData| {
            // 視野行列
            let position = glm::Vec3::new(5.0, 2.0, 5.0);
            let look_at = glm::Vec3::zeros();
            let up = glm::Vec3::new(0.0, 1.0, 0.0);
            let view_matrix = glm::look_at(&position, &look_at, &up);

            // 投影行列
            let field_of_view = std::f32::consts::PI / 4.0;
            let aspect = 640.0 / 480.0;
            let near = 0.1;
            let far = 100.0;
            let projection_matrix = glm::perspective(aspect, field_of_view, near, far);

            // 定数バッファに書き込み
            data.projection_view_matrix = projection_matrix * view_matrix;
        });
        let light_constant_buffer = TBufferBuilder::<TApi>::new()
            .enable_constant_buffer()
            .with_size(std::mem::size_of::<LightData>())
            .build(device);
        light_constant_buffer.map_mut(|data: &mut LightData| {
            let direction = glm::Vec3::new(-1.0, -1.0, -1.0).normalize();
            data.light_direction = glm::Vec4::new(direction[0], direction[1], direction[2], 1.0);
            data.color = glm::Vec4::new(1.0, 1.0, 1.0, 1.0);
        });

        // 頂点ステート
        let vertex_state =
            TVertexStateBuilder::<TApi>::new()
                .set_vertex_attribute_states(
                    [
                        VertexAttributeStateInfo::new()
                            .set_buffer_index(0)
                            .set_format(AttributeFormat::Float32_32_32)
                            .set_offset(0)
                            .set_slot(0),
                        VertexAttributeStateInfo::new()
                            .set_buffer_index(0)
                            .set_format(AttributeFormat::Float32_32_32)
                            .set_offset((std::mem::size_of::<f32>() * 3) as i64)
                            .set_slot(1),
                    ]
                    .into_iter(),
                )
                .set_vertex_buffer_states(
                    [VertexBufferStateInfo::new()
                        .set_stride((std::mem::size_of::<f32>() * 6) as i64)]
                    .into_iter(),
                )
                .build(device);

        let depth_buffer = TTextureBuilder::<TApi>::new()
            .enable_depth_buffer()
            .with_format(ImageFormat::D32)
            .with_size(1280, 960)
            .build(device);
        let depth_stencil_view =
            TApi::DepthStencilView::new(device, &DepthStencilStateInfo::new(), &depth_buffer);

        // コマンドバッファ
        let command_buffer = TCommandBufferBuilder::<TApi>::new().build(device);

        Self {
            depth_buffer,
            depth_stencil_view,
            basic_shader: shader,
            view_constant_buffer: constnat_buffer,
            light_constant_buffer,
            vertex_state,
            command_buffers: [command_buffer],
        }
    }

    pub fn make_command(&mut self, scene: &Scene<TApi>, color_target_view: &TApi::ColorTargetView) {
        let geometry_container = scene.get_geometry_container();

        // TODO: 可変ジオメトリ対応
        let vertex_buffer = &geometry_container.get_vertex_buffers()[0];
        let index_buffer = &geometry_container.get_index_buffers()[0];
        let constant_buffer = &geometry_container.get_constant_buffers()[0];
        let index_count = geometry_container.get_index_counts()[0];

        self.command_buffers[0].begin();
        self.command_buffers[0]
            .set_render_targets(&[color_target_view], Some(&self.depth_stencil_view));
        self.command_buffers[0].set_shader(&self.basic_shader);
        self.command_buffers[0].set_constant_buffer(0, constant_buffer);
        self.command_buffers[0].set_constant_buffer(1, &self.view_constant_buffer);
        self.command_buffers[0].set_constant_buffer(2, &self.light_constant_buffer);
        self.command_buffers[0].set_vertex_state(&self.vertex_state);
        self.command_buffers[0].set_vertex_buffer(0, vertex_buffer);
        self.command_buffers[0].draw_indexed(
            PrimitiveTopology::TriangleList,
            IndexFormat::Uint32,
            index_buffer,
            index_count,
            0, /*base_vertex*/
        );
        self.command_buffers[0].end();
    }

    pub fn get_command_buffers(&self) -> &[TApi::CommandBuffer] {
        &self.command_buffers
    }

    pub fn set_view_matrix(&self, view_matrix: &glm::Mat4x4) {
        // 定数バッファ
        self.view_constant_buffer.map_mut(|data: &mut ViewData| {
            // 投影行列
            let field_of_view = std::f32::consts::PI / 4.0;
            let aspect = 640.0 / 480.0;
            let near = 0.1;
            let far = 100.0;
            let projection_matrix = glm::perspective(aspect, field_of_view, near, far);

            // 定数バッファに書き込み
            data.projection_view_matrix = projection_matrix * view_matrix;
        });
    }
}
