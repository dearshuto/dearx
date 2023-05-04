use crate::{IDrawInfo, IGraphicsObjectId};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id {
    pub(crate) index: i32,
}
impl IGraphicsObjectId for Id {}

pub struct DrawInfo {
    pub(crate) pipeline_id: Id,
    pub(crate) vertex_buffer_ids: Vec<Id>,
    pub(crate) draw_command_info: Id,
}

impl<'a> IDrawInfo for &'a DrawInfo {
    type TId = Id;

    fn get_pipeline_id(&self) -> Self::TId {
        self.pipeline_id
    }

    fn get_vertex_buffer_ids(&self) -> &[Self::TId] {
        &self.vertex_buffer_ids
    }

    fn get_draw_command_info_id(&self) -> Self::TId {
        self.draw_command_info
    }
}
