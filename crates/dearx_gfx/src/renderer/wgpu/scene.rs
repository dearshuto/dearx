use crate::{renderer::SceneObject, DrawCommandInfo, IScene};

use super::{DrawInfo, Id};

pub struct Scene {
    render_pipeline: Vec<wgpu::RenderPipeline>,
    vertex_buffers: Vec<wgpu::Buffer>,
    draw_infos: Vec<DrawInfo>,
}

impl Scene {
    pub fn new_graphics(
        _device: &wgpu::Device,
        _fragment_format: wgpu::TextureFormat,
        scene_object: SceneObject<wgpu::Buffer, wgpu::RenderPipeline>,
    ) -> Self {
        Self {
            render_pipeline: scene_object.pipelines,
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
