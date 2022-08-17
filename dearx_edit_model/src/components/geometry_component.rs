use dearx_macro::Immutable;
use std::sync::Arc;

#[derive(Immutable)]
pub struct GeometryComponent {
    pub transform_index: u32,
}
