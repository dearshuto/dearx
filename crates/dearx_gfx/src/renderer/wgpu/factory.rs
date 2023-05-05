use wgpu::{util::DeviceExt, Device};

use crate::serializer::{
    CreateBufferDescriptor, CreateDescriptorPoolDescriptor, CreateRenderPipelineDescriptor,
    IFactory,
};

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
    type TDescriptorPool = wgpu::BindGroupLayout;

    fn create_buffer(&self, descriptor: &CreateBufferDescriptor) -> Self::TBuffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: descriptor.data,
                usage: sjgfx_wgpu::util::convert_to_buffer_usage(descriptor.gpu_access),
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

        let pipeline_layout = sjgfx_wgpu::util::create_pipeline_layout(
            self.device,
            descriptor.vertex_shader,
            descriptor.pixel_shader.unwrap(),
        );

        let vertex_shader_reflection =
            sjgfx_util::ShaderReflection::new_from_biinary(descriptor.vertex_shader);
        let attributes = vertex_shader_reflection
            .entry_point
            .attribures()
            .iter()
            .map(|attribute| wgpu::VertexAttribute {
                format: sjgfx_wgpu::util::convert_attribute_format(attribute.format()),
                offset: attribute.offset() as u64,
                shader_location: attribute.location(),
            })
            .collect::<Vec<wgpu::VertexAttribute>>();

        self.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader_module,
                    entry_point: "main",
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: descriptor.vertex_buffer_offsets[0] as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &attributes,
                    }],
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: fragment_state,
                multiview: None,
            })
    }

    fn create_descriptor_pool(
        &self,
        descriptor: &CreateDescriptorPoolDescriptor,
    ) -> Self::TDescriptorPool {
        sjgfx_wgpu::util::create_bind_group_layout(
            self.device,
            descriptor.vertex_shader,
            descriptor.pixel_shader,
        )
    }
}
