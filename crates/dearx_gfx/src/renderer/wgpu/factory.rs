use wgpu::{util::DeviceExt, Device};

use crate::serializer::{CreateBufferDescriptor, CreateRenderPipelineDescriptor, IFactory};

pub struct Factory<'a> {
    device: &'a wgpu::Device,
    format: wgpu::TextureFormat,
}

impl<'a> Factory<'a> {
    pub fn new(device: &'a Device, render_target_format: wgpu::TextureFormat) -> Self {
        Self {
            device,
            format: render_target_format,
        }
    }
}

impl<'a> IFactory for Factory<'a> {
    type TBuffer = wgpu::Buffer;
    type TRenderPipeline = wgpu::RenderPipeline;

    fn create_buffer(&self, descriptor: &CreateBufferDescriptor) -> Self::TBuffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: descriptor.data,
                usage: wgpu::BufferUsages::VERTEX,
            })
    }

    fn create_render_pipeline(
        &self,
        descriptor: &CreateRenderPipelineDescriptor,
    ) -> Self::TRenderPipeline {
        // シェーダーモジュール
        let vertex_shader_module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::util::make_spirv(descriptor.vertex_shader),
            });
        let pixel_shader_module = descriptor.pixel_shader.map(|pixel_shader_biinary| {
            self.device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::util::make_spirv(pixel_shader_biinary),
                })
        });

        let targets = &[Some(wgpu::ColorTargetState {
            format: self.format,
            blend: None,
            write_mask: wgpu::ColorWrites::all(),
        })];
        let fragment_state =
            pixel_shader_module
                .as_ref()
                .map(|pixel_shader_module| wgpu::FragmentState {
                    module: &pixel_shader_module,
                    entry_point: "main",
                    targets,
                });

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        self.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                fragment: fragment_state,
                multiview: None,
            })
    }
}
