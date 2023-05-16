use crate::{
    renderer::scene::IContainer, DrawCommandInfo, IDrawInfo, IGraphicsObjectId, SceneObject,
};

pub struct VectorContainer<TPipeline, TDescriptorPool, TBuffer> {
    render_pipeline: Vec<TPipeline>,
    descriptor_pools: Vec<TDescriptorPool>,
    vertex_buffers: Vec<TBuffer>,
    #[allow(dead_code)]
    constant_buffers: Vec<TBuffer>,
    #[allow(dead_code)]
    draw_commands: Vec<DrawCommandInfo>,
    draw_infos: Vec<VectorDrawInfo>,
}

pub struct VectorDrawInfo {
    pub(crate) pipeline_id: i32,
    pub(crate) descriptor_pool_id: i32,
    pub(crate) vertex_buffer_ids: Vec<i32>,
    pub(crate) draw_command_info: i32,
}

impl IGraphicsObjectId for i32 {}

impl<'a> IDrawInfo for &'a VectorDrawInfo {
    type TId = i32;

    fn get_pipeline_id(&self) -> Self::TId {
        self.pipeline_id
    }

    fn get_descriptor_pool_id(&self) -> Self::TId {
        self.descriptor_pool_id
    }

    fn get_vertex_buffer_ids(&self) -> &[Self::TId] {
        &self.vertex_buffer_ids
    }

    fn get_draw_command_info_id(&self) -> Self::TId {
        self.draw_command_info
    }
}

impl<TPipeline, TDescriptorPool, TBuffer> IContainer
    for VectorContainer<TPipeline, TDescriptorPool, TBuffer>
{
    type Id = i32;
    type TPipeline = TPipeline;
    type TDescriptorPool = TDescriptorPool;
    type TBuffer = TBuffer;
    type TDrawInfo = VectorDrawInfo;

    fn from_scene_object(
        scene_object: SceneObject<Self::TBuffer, Self::TPipeline, Self::TDescriptorPool>,
    ) -> Self {
        Self {
            render_pipeline: scene_object.pipelines,
            descriptor_pools: scene_object.descriptor_pool,
            vertex_buffers: scene_object.vertex_buffers,
            constant_buffers: scene_object.constant_buffers,
            draw_commands: vec![DrawCommandInfo::Draw(3), DrawCommandInfo::Draw(36)],
            draw_infos: vec![
                VectorDrawInfo {
                    pipeline_id: 1,
                    descriptor_pool_id: 1,
                    vertex_buffer_ids: vec![2],
                    draw_command_info: 1,
                },
                VectorDrawInfo {
                    pipeline_id: 0,
                    descriptor_pool_id: 0,
                    vertex_buffer_ids: vec![0],
                    draw_command_info: 0,
                },
                VectorDrawInfo {
                    pipeline_id: 0,
                    descriptor_pool_id: 0,
                    vertex_buffer_ids: vec![1],
                    draw_command_info: 0,
                },
            ],
        }
    }

    fn get_pipeline(&self, id: &Self::Id) -> &Self::TPipeline {
        &self.render_pipeline[*id as usize]
    }

    fn get_descriptor_pool(&self, id: &Self::Id) -> &Self::TDescriptorPool {
        &self.descriptor_pools[*id as usize]
    }

    fn get_vertex_buffer(&self, id: &Self::Id) -> &Self::TBuffer {
        &self.vertex_buffers[*id as usize]
    }

    fn get_draw_command(&self, id: &Self::Id) -> &DrawCommandInfo {
        &self.draw_commands[*id as usize]
    }

    fn get_draw_infos(&self) -> &[Self::TDrawInfo] {
        &self.draw_infos
    }
}
