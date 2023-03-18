use std::path::PathBuf;
use std::sync::Arc;

use dearx_macro::Immutable;

#[derive(Immutable)]
pub struct StaticMeshComponent {
    pub file_path: PathBuf,
}
