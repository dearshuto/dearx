use std::sync::Arc;
use sje_immutable_macro::Immutable;

#[derive(Immutable)]
pub struct GeometryComponent
{
    pub transform_index: u32,
}
