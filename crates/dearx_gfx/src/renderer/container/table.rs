use std::collections::HashMap;

use crate::{
    renderer::scene::IContainer, DrawCommandInfo, IDrawInfo, IGraphicsObjectId, SceneObject,
};

pub struct TableDrawInfo {
    pub(crate) pipeline_id: uuid::Uuid,
    pub(crate) descriptor_pool_id: uuid::Uuid,
    pub(crate) vertex_buffer_ids: Vec<uuid::Uuid>,
    pub(crate) draw_command_info: uuid::Uuid,
}

impl IGraphicsObjectId for uuid::Uuid {}

impl<'a> IDrawInfo for &'a TableDrawInfo {
    type TId = uuid::Uuid;

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

pub struct TableContainer<TPipeline, TDescriptorPool, TBuffer> {
    render_pipeline: HashMap<uuid::Uuid, TPipeline>,
    descriptor_pools: HashMap<uuid::Uuid, TDescriptorPool>,
    vertex_buffers: HashMap<uuid::Uuid, TBuffer>,
    #[allow(dead_code)]
    constant_buffers: HashMap<uuid::Uuid, TBuffer>,
    #[allow(dead_code)]
    draw_commands: HashMap<uuid::Uuid, DrawCommandInfo>,
    #[allow(dead_code)]
    draw_infos: Vec<TableDrawInfo>,
}

impl<TPipeline, TDescriptorPool, TBuffer> IContainer
    for TableContainer<TPipeline, TDescriptorPool, TBuffer>
{
    type Id = uuid::Uuid;
    type TPipeline = TPipeline;
    type TDescriptorPool = TDescriptorPool;
    type TBuffer = TBuffer;
    type TDrawInfo = TableDrawInfo;

    fn from_scene_object(
        scene_object: SceneObject<Self::TBuffer, Self::TPipeline, Self::TDescriptorPool>,
    ) -> Self {
        let pipeline_ids = (0..scene_object.pipelines.len())
            .map(|_| uuid::Uuid::new_v4())
            .collect::<Vec<uuid::Uuid>>();
        let render_pipeline = scene_object
            .pipelines
            .into_iter()
            .enumerate()
            .map(|(index, item)| (pipeline_ids[index], item))
            .collect::<HashMap<uuid::Uuid, TPipeline>>();

        let descriptor_pool_ids = (0..scene_object.descriptor_pool.len())
            .map(|_| uuid::Uuid::new_v4())
            .collect::<Vec<uuid::Uuid>>();
        let descriptor_pools = scene_object
            .descriptor_pool
            .into_iter()
            .enumerate()
            .map(|(index, item)| (descriptor_pool_ids[index], item))
            .collect::<HashMap<uuid::Uuid, TDescriptorPool>>();

        let constant_buffer_ids = (0..scene_object.constant_buffers.len())
            .map(|_| uuid::Uuid::new_v4())
            .collect::<Vec<uuid::Uuid>>();
        let constant_buffers = scene_object
            .constant_buffers
            .into_iter()
            .enumerate()
            .map(|(index, item)| (constant_buffer_ids[index], item))
            .collect::<HashMap<uuid::Uuid, TBuffer>>();

        let vertex_buffer_ids = (0..scene_object.vertex_buffers.len())
            .map(|_| uuid::Uuid::new_v4())
            .collect::<Vec<uuid::Uuid>>();
        let vertex_buffers = scene_object
            .vertex_buffers
            .into_iter()
            .enumerate()
            .map(|(index, item)| (vertex_buffer_ids[index], item))
            .collect::<HashMap<uuid::Uuid, TBuffer>>();

        let draw_command_ids = [uuid::Uuid::new_v4(), uuid::Uuid::new_v4()];
        let draw_commands = [DrawCommandInfo::Draw(3), DrawCommandInfo::Draw(36)]
            .into_iter()
            .enumerate()
            .map(|(index, item)| (draw_command_ids[index], item))
            .collect::<HashMap<uuid::Uuid, DrawCommandInfo>>();

        Self {
            render_pipeline,
            descriptor_pools,
            constant_buffers,
            vertex_buffers,
            draw_commands,
            draw_infos: vec![
                TableDrawInfo {
                    pipeline_id: pipeline_ids[1],
                    descriptor_pool_id: descriptor_pool_ids[1],
                    vertex_buffer_ids: vec![vertex_buffer_ids[2]],
                    draw_command_info: draw_command_ids[1],
                },
                TableDrawInfo {
                    pipeline_id: pipeline_ids[0],
                    descriptor_pool_id: descriptor_pool_ids[0],
                    vertex_buffer_ids: vec![vertex_buffer_ids[0]],
                    draw_command_info: draw_command_ids[0],
                },
                TableDrawInfo {
                    pipeline_id: pipeline_ids[0],
                    descriptor_pool_id: descriptor_pool_ids[0],
                    vertex_buffer_ids: vec![vertex_buffer_ids[1]],
                    draw_command_info: draw_command_ids[0],
                },
            ],
        }
    }

    fn get_pipeline(&self, id: &Self::Id) -> &Self::TPipeline {
        self.render_pipeline.get(id).as_ref().unwrap()
    }

    fn get_descriptor_pool(&self, id: &Self::Id) -> &Self::TDescriptorPool {
        self.descriptor_pools.get(id).as_ref().unwrap()
    }

    fn get_vertex_buffer(&self, id: &Self::Id) -> &Self::TBuffer {
        self.vertex_buffers.get(id).as_ref().unwrap()
    }

    fn get_draw_command(&self, id: &Self::Id) -> &DrawCommandInfo {
        self.draw_commands.get(id).as_ref().unwrap()
    }

    fn get_draw_infos(&self) -> &[Self::TDrawInfo] {
        // TODO
        &[]
    }
}
