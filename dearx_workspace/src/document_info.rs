use std::sync::Arc;

use dearx_macro::Immutable;

#[derive(Immutable)]
pub struct DocumentInfo<T> {
    pub content: Arc<T>,
}
