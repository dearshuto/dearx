use std::sync::Arc;
use sje_immutable_macro::Immutable;

use crate::Float3;

#[derive(Immutable)]
pub struct TransformComponent
{
    pub translation: Float3,
    pub rotation: Float3,
    pub scale: Float3,
    pub parent_transform_index: Option<u32>,
}
