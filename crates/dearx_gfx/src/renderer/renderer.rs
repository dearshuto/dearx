extern crate nalgebra_glm as glm;

#[derive(Clone)]
pub enum DrawCommandInfo {
    Draw(i32),
    DrawInstanced(i32, i32),
}

pub trait IGraphicsObjectId: Copy + Eq {}

pub trait IDrawInfo {
    type TId: IGraphicsObjectId;

    fn get_pipeline_id(&self) -> Self::TId;

    fn get_descriptor_pool_id(&self) -> Self::TId;

    fn get_vertex_buffer_ids(&self) -> &[Self::TId];

    fn get_draw_command_info_id(&self) -> Self::TId;
}

pub trait IScene {
    type TBuffer;
    type TPipeline;
    type TDescriptorPool;
    type TGraphicsObjectId;
    type TEditId;

    fn get_pipeline(&self, id: Self::TGraphicsObjectId) -> &Self::TPipeline;

    fn get_descriptor_pool(&self, id: Self::TGraphicsObjectId) -> &Self::TDescriptorPool;

    fn get_vertex_buffer(&self, id: Self::TGraphicsObjectId) -> &Self::TBuffer;

    fn get_draw_command(&self, id: Self::TGraphicsObjectId) -> &DrawCommandInfo;
}

pub trait ICommandBuffer<'a> {
    type TBuffer;
    type TPipeline;
    type TDescriptorPool;

    fn set_pipeline(&mut self, pipeline: &'a Self::TPipeline);

    fn set_descriptor_pool(&mut self, descriptor_pool: &'a Self::TDescriptorPool);

    fn set_vertex_buffer(&mut self, index: i32, buffer_ref: &'a Self::TBuffer);

    fn draw(&mut self, count: i32);
}

#[derive(Default)]
pub struct Renderer {}

impl Renderer {
    pub fn render<'a, TCommandBuffer, TScene, TDrawInfo, TIterator>(
        &self,
        command_buffer: &mut TCommandBuffer,
        scene: &'a TScene,
        iterator: TIterator,
    ) where
        TCommandBuffer: ICommandBuffer<'a> + 'a,
        TScene: IScene<
            TBuffer = TCommandBuffer::TBuffer,
            TPipeline = TCommandBuffer::TPipeline,
            TDescriptorPool = TCommandBuffer::TDescriptorPool,
            TGraphicsObjectId = TDrawInfo::TId,
        >,
        TDrawInfo: IDrawInfo,
        TIterator: IntoIterator<Item = TDrawInfo>,
    {
        for draw_info in iterator {
            self.render_impl(command_buffer, scene, &draw_info);
        }
    }

    fn render_impl<'a, TDrawInfo, TScene, TCommandBuffer>(
        &self,
        command_buffer: &mut TCommandBuffer,
        scene: &'a TScene,
        draw_info: &TDrawInfo,
    ) where
        TCommandBuffer: ICommandBuffer<'a> + 'a,
        TDrawInfo: IDrawInfo,
        TScene: IScene<
            TBuffer = TCommandBuffer::TBuffer,
            TPipeline = TCommandBuffer::TPipeline,
            TDescriptorPool = TCommandBuffer::TDescriptorPool,
            TGraphicsObjectId = TDrawInfo::TId,
        >,
    {
        // パイプライン
        let pipeline_id = draw_info.get_pipeline_id();
        let pipeline = scene.get_pipeline(pipeline_id);
        command_buffer.set_pipeline(pipeline);

        // デスクリプター
        let descriptor_pool_id = draw_info.get_descriptor_pool_id();
        let descriptor_pool = scene.get_descriptor_pool(descriptor_pool_id);
        command_buffer.set_descriptor_pool(descriptor_pool);

        // 頂点バッファー
        for id in draw_info.get_vertex_buffer_ids() {
            let vertex_buffer = scene.get_vertex_buffer(*id);
            command_buffer.set_vertex_buffer(0, vertex_buffer);
        }

        // 描画コマンド
        let draw_command_id = draw_info.get_draw_command_info_id();
        let draw_command = scene.get_draw_command(draw_command_id);
        match draw_command {
            DrawCommandInfo::Draw(vertex_count) => command_buffer.draw(*vertex_count),
            DrawCommandInfo::DrawInstanced(_, _) => { /* todo */ }
        }
    }
}
