#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut compiler = sjgfx_util::ShaderCompiler::new();

    // ベーシック
    compiler.build_graphics_shader(&"resources/shaders/basic.vs", &"resources/shaders/basic.fs");

    // gRPC
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/dearx.proto"], &Vec::<std::path::PathBuf>::new())
        .unwrap();
}
