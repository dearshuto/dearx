use dearx_gfx::{wgpu::Scene, ICommandBuffer, Renderer};

#[derive(Default)]
struct TestCommandBuffer;
impl<'a> ICommandBuffer<'a> for TestCommandBuffer {
    type TBuffer = wgpu::Buffer;
    type TPipeline = wgpu::RenderPipeline;

    fn set_pipeline(&mut self, _pipeline: &Self::TPipeline) {}

    fn set_vertex_buffer(&mut self, _index: i32, _buffer_ref: &'a Self::TBuffer) {}

    fn draw(&mut self, _count: i32) {}
}

#[test]
fn new() {
    let mut command_buffer = TestCommandBuffer::default();
    let scene = Scene::new();
    let renderer = Renderer::default();
    renderer.render(&mut command_buffer, &scene, scene.enumerate_draw_info());
}
