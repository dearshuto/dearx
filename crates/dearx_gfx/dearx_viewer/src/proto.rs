// tonic-build で生成したファイルをクレートないに取り込むための実装

// なんでこんなファイル名になるのか不明だが、import したファイル名はこれ
// MEMO: 生成されたファイルは pub struct
tonic::include_proto!("_");

mod detail {
    // import した型は suprt::Xyz と実装されているので、ビルドが通るように mod を区切っておく
    tonic::include_proto!("dearx_viewer");
}

// detail モジュールはビルドは tonic-build が生成したファイルをビルドするためだけの役割
// 利便性のために公開するときは detail なしにする
pub use detail::{
    CreateReply, CreateRequest, DeleteReply, DeleteRequest, GetMeshReply, GetMeshRequest, GetReply,
    GetRequest, GetSceneInfoReply, GetSceneInfoRequest, GetShaderReply, GetShaderRequest,
    UpdateReply, UpdateRequest,
};
