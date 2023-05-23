mod table;
mod vector;

pub use table::{TableContainer, TableDrawInfo};
pub use vector::{VectorContainer, VectorDrawInfo};

use crate::{DrawCommandInfo, SceneObject};

pub trait IContainer {
    type Id;
    type TPipeline;
    type TDescriptorPool;
    type TBuffer;
    type TDrawInfo;

    fn from_scene_object(
        scene_object: SceneObject<Self::TBuffer, Self::TPipeline, Self::TDescriptorPool>,
    ) -> Self;

    fn get_pipeline(&self, id: &Self::Id) -> &Self::TPipeline;

    fn get_descriptor_pool(&self, id: &Self::Id) -> &Self::TDescriptorPool;

    fn get_vertex_buffer(&self, id: &Self::Id) -> &Self::TBuffer;

    fn get_constant_buffer_mut(&mut self, id: &Self::Id) -> &mut Self::TBuffer;

    fn get_draw_command(&self, id: &Self::Id) -> &DrawCommandInfo;

    fn get_draw_infos(&self) -> &[Self::TDrawInfo];
}
