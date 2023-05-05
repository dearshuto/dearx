mod deserializer;

// モジュール名と同名のファイルがあると出る警告を抑制
#[allow(clippy::module_inception)]
mod serializer;

pub use deserializer::{
    deserialize, CreateBufferDescriptor, CreateRenderPipelineDescriptor, IFactory,
};
pub use serializer::serialize;
