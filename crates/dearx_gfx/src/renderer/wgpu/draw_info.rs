use crate::{IDrawInfo, IGraphicsObjectId};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id {
    index: i32,
}
impl IGraphicsObjectId for Id {}

pub struct DrawInfo {}

impl IDrawInfo for &DrawInfo {
    type TId = Id;
    type TIterator = std::vec::IntoIter<Self::TId>;

    fn get_pipeline_id(&self) -> Self::TId {
        // TODO
        Id { index: 0 }
    }

    fn get_vertex_buffer_ids(&self) -> Self::TIterator {
        vec![Id { index: 0 }].into_iter()
    }

    fn get_draw_command_info_id(&self) -> Self::TId {
        // TODO
        Id { index: 0 }
    }
}
