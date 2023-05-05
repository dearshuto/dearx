use crate::{renderer::SceneObject, DrawCommandInfo, IScene};

use super::{DrawInfo, Id};

pub struct Scene {
    render_pipeline: Vec<wgpu::RenderPipeline>,
    vertex_buffers: Vec<wgpu::Buffer>,
    draw_infos: Vec<DrawInfo>,
}

impl Scene {
    pub fn new_graphics(
        device: &wgpu::Device,
        fragment_format: wgpu::TextureFormat,
        scene_object: SceneObject<wgpu::Buffer>,
    ) -> Self {
        // シェーダーバイナリ
        let vertex_shader_source = include_str!("../../../resources/shaders/triangle.vs");
        let pixel_shader_source = include_str!("../../../resources/shaders/triangle.fs");
        let mut compiler = sjgfx_util::ShaderCompiler::new();
        let vertex_shader_binary =
            compiler.create_binary(vertex_shader_source, sjgfx_util::ShaderStage::Vertex);
        let pixel_shader_binary =
            compiler.create_binary(pixel_shader_source, sjgfx_util::ShaderStage::Pixel);

        // シェーダーモジュール
        let vertex_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::util::make_spirv(&vertex_shader_binary),
        });
        let pixel_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::util::make_spirv(&pixel_shader_binary),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader_module,
                entry_point: "main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: (std::mem::size_of::<f32>() * 3) as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }],
                }],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &pixel_shader_module,
                entry_point: "main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: fragment_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
        });

        Self {
            render_pipeline: vec![render_pipeline],
            vertex_buffers: scene_object.vertex_buffers,
            draw_infos: vec![
                DrawInfo {
                    pipeline_id: Id { index: 0 },
                    vertex_buffer_ids: vec![Id { index: 0 }],
                    draw_command_info: Id { index: 0 },
                },
                DrawInfo {
                    pipeline_id: Id { index: 0 },
                    vertex_buffer_ids: vec![Id { index: 1 }],
                    draw_command_info: Id { index: 0 },
                },
            ],
        }
    }

    pub fn new() -> Self {
        Self {
            render_pipeline: Vec::default(),
            vertex_buffers: Vec::default(),
            draw_infos: Vec::default(),
        }
    }

    pub fn enumerate_draw_info(&self) -> &[DrawInfo] {
        &self.draw_infos
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl IScene for Scene {
    type TBuffer = wgpu::Buffer;
    type TPipeline = wgpu::RenderPipeline;
    type TGraphicsObjectId = Id;

    fn get_pipeline(&self, id: Self::TGraphicsObjectId) -> &Self::TPipeline {
        let index = id.index;
        &self.render_pipeline[index as usize]
    }

    fn get_vertex_buffer(&self, id: Self::TGraphicsObjectId) -> &Self::TBuffer {
        let index = id.index;
        &self.vertex_buffers[index as usize]
    }

    fn get_draw_info(&self, _id: Self::TGraphicsObjectId) -> crate::DrawCommandInfo {
        DrawCommandInfo::Draw(3)
    }
}
