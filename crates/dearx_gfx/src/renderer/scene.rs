use crate::{DrawCommandInfo, IScene, SceneObject};

use super::{
    container::{IContainer, TableDrawInfo, VectorDrawInfo},
    TableContainer, VectorContainer,
};

pub struct Scene<
    TRenderPipeline,
    TDescriptorPool,
    TBuffer,
    TContainer = VectorContainer<TRenderPipeline, TDescriptorPool, TBuffer>,
> where
    TContainer: IContainer<
        TPipeline = TRenderPipeline,
        TDescriptorPool = TDescriptorPool,
        TBuffer = TBuffer,
    >,
{
    container: TContainer,
}

pub struct PropertyId<T> {
    _marker: std::marker::PhantomData<T>,
}

// 実装
impl<TBuffer, TPipeline, TDescriptorPool, TContainer>
    Scene<TPipeline, TDescriptorPool, TBuffer, TContainer>
where
    TContainer:
        IContainer<TBuffer = TBuffer, TPipeline = TPipeline, TDescriptorPool = TDescriptorPool>,
{
    pub fn edit_param(&mut self, _id: &PropertyId<TContainer::Id>) {}
}

// Vector 実装の特殊処理
impl<TBuffer, TPipeline, TDescriptorPool>
    Scene<TPipeline, TDescriptorPool, TBuffer, VectorContainer<TPipeline, TDescriptorPool, TBuffer>>
{
    pub fn from_scene_object(
        scene_object: SceneObject<TBuffer, TPipeline, TDescriptorPool>,
    ) -> Self {
        let container = VectorContainer::from_scene_object(scene_object);
        Self { container }
    }

    pub fn get_draw_infos(&self) -> &[VectorDrawInfo] {
        self.container.get_draw_infos()
    }
}

// Table 実装の特殊処理
impl<TBuffer, TPipeline, TDescriptorPool>
    Scene<TPipeline, TDescriptorPool, TBuffer, TableContainer<TPipeline, TDescriptorPool, TBuffer>>
{
    pub fn new_table_scene(scene_object: SceneObject<TBuffer, TPipeline, TDescriptorPool>) -> Self {
        let container = TableContainer::from_scene_object(scene_object);
        Self { container }
    }

    pub fn get_draw_infos(&self) -> &[TableDrawInfo] {
        self.container.get_draw_infos()
    }
}

impl<TBuffer, TPipeline, TDescriptorPool, TContainer> IScene
    for Scene<TPipeline, TDescriptorPool, TBuffer, TContainer>
where
    TContainer:
        IContainer<TBuffer = TBuffer, TPipeline = TPipeline, TDescriptorPool = TDescriptorPool>,
{
    type TBuffer = TBuffer;
    type TPipeline = TPipeline;
    type TDescriptorPool = TDescriptorPool;
    type TGraphicsObjectId = TContainer::Id;
    type TEditId = i32;

    fn get_pipeline(&self, id: Self::TGraphicsObjectId) -> &Self::TPipeline {
        self.container.get_pipeline(&id)
    }

    fn get_descriptor_pool(&self, id: Self::TGraphicsObjectId) -> &Self::TDescriptorPool {
        self.container.get_descriptor_pool(&id)
    }

    fn get_vertex_buffer(&self, id: Self::TGraphicsObjectId) -> &Self::TBuffer {
        self.container.get_vertex_buffer(&id)
    }

    fn get_draw_command(&self, id: Self::TGraphicsObjectId) -> &DrawCommandInfo {
        self.container.get_draw_command(&id)
    }
}
