use super::{DrawInfo, Id};
use crate::{renderer::SceneObject, DrawCommandInfo, IScene};

pub struct Scene {
    render_pipeline: Vec<wgpu::RenderPipeline>,
    bind_group: Vec<wgpu::BindGroup>,
    vertex_buffers: Vec<wgpu::Buffer>,

    #[allow(dead_code)]
    constant_buffers: Vec<wgpu::Buffer>,

    draw_infos: Vec<DrawInfo>,
}

impl Scene {
    pub fn new_graphics(
        _device: &wgpu::Device,
        _fragment_format: wgpu::TextureFormat,
        scene_object: SceneObject<wgpu::Buffer, wgpu::RenderPipeline, wgpu::BindGroup>,
    ) -> Self {
        Self {
            render_pipeline: scene_object.pipelines,
            bind_group: scene_object.descriptor_pool,
            vertex_buffers: scene_object.vertex_buffers,
            constant_buffers: scene_object.constant_buffers,
            draw_infos: vec![
                DrawInfo {
                    pipeline_id: Id { index: 0 },
                    descriptor_pool_id: Id { index: 0 },
                    vertex_buffer_ids: vec![Id { index: 0 }],
                    draw_command_info: Id { index: 0 },
                },
                DrawInfo {
                    pipeline_id: Id { index: 0 },
                    descriptor_pool_id: Id { index: 0 },
                    vertex_buffer_ids: vec![Id { index: 1 }],
                    draw_command_info: Id { index: 0 },
                },
            ],
        }
    }

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn deserialize(data: &[u8]) -> Self {
        let mut stream_reader = usd_rs::StreamReader::new(data);
        let _reader = usd_rs::AsciiReader::new(&mut stream_reader);
        todo!()
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
    type TDescriptorPool = wgpu::BindGroup;
    type TGraphicsObjectId = Id;

    fn get_pipeline(&self, id: Self::TGraphicsObjectId) -> &Self::TPipeline {
        let index = id.index;
        &self.render_pipeline[index as usize]
    }

    fn get_descriptor_pool(&self, id: Self::TGraphicsObjectId) -> &Self::TDescriptorPool {
        let index = id.index;
        &self.bind_group[index as usize]
    }

    fn get_vertex_buffer(&self, id: Self::TGraphicsObjectId) -> &Self::TBuffer {
        let index = id.index;
        &self.vertex_buffers[index as usize]
    }

    fn get_draw_info(&self, _id: Self::TGraphicsObjectId) -> crate::DrawCommandInfo {
        DrawCommandInfo::Draw(3)
    }
}
