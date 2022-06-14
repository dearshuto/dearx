fn main() {
    let mut compiler = sjgfx_util::ShaderCompiler::new();

    // グリッド
    compiler.build_graphics_shader(&"resources/shaders/grid.vs", &"resources/shaders/grid.fs");
}
