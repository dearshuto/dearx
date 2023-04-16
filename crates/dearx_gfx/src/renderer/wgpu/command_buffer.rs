use crate::ICommandBuffer;

impl<'a> ICommandBuffer<'a> for wgpu::RenderPass<'a> {
    type TBuffer = wgpu::Buffer;
    type TPipeline = wgpu::RenderPipeline;

    fn set_pipeline(&mut self, pipeline: &'a Self::TPipeline) {
        self.set_pipeline(pipeline);
    }

    fn set_vertex_buffer(&mut self, index: i32, buffer: &'a Self::TBuffer) {
        self.set_vertex_buffer(index as u32, buffer.slice(..));
    }

    fn draw(&mut self, count: i32) {
        self.draw(0..(count as u32), 0..1 /*インスタンス*/);
    }
}
