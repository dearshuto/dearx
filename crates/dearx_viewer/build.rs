fn main() {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .type_attribute(
            "dearx_viewer.GetRequest",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "dearx_viewer.GetMeshRequest",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "dearx_viewer.GetSceneInfoRequest",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "dearx_viewer.GetShaderRequest",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .compile(
            &["proto/dearx_viewer.proto"],
            &Vec::<std::path::PathBuf>::new(),
        )
        .unwrap();
}
