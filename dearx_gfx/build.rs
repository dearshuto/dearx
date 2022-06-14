fn main() {
    let mut compiler = sjgfx_util::ShaderCompiler::new();

    // ベーシック
    compiler.build_graphics_shader(
        &"resources/shaders/basic.vs",
        &"resources/shaders/basic.fs",
    );
}
