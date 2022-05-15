use std::path::PathBuf;
use std::sync::Arc;

use sje_generator_macro::Immutable;

#[derive(Immutable)]
pub struct StaticMeshComponent {
    pub file_path: PathBuf,
}
