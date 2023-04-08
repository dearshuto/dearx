fn main() {
    let mut compiler = sjgfx_util::ShaderCompiler::new();

    // ベーシック
    compiler.build_graphics_shader(&"resources/shaders/basic.vs", &"resources/shaders/basic.fs");

    // gRPC
    tonic_build::configure()
        .build_server(false)
        .build_client(false)
        .compile(&["proto/dearx.proto"], &Vec::<std::path::PathBuf>::new())
        .unwrap();
}
