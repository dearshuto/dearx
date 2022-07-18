use sjgfx::{api::IApi, TBufferBuilder, TCommandBufferBuilder};
use sjgfx_interface::{ICommandBuffer, IQueue};

pub struct DesignView<TApi, TViewModel>
where
    TApi: IApi,
    TViewModel: IDesignViewViewModel<TApi>,
{
    view_model: TViewModel,
    command_buffer: TApi::CommandBuffer,
    constant_buffer: TApi::Buffer,
    _marker: std::marker::PhantomData<TApi>,
}

impl<TApi, TViewModel> DesignView<TApi, TViewModel>
where
    TApi: IApi,
    TViewModel: IDesignViewViewModel<TApi>,
{
    pub fn new(device: &mut TApi::Device, view_model: TViewModel) -> Self {
        let command_buffer = TCommandBufferBuilder::<TApi>::new().build(device);
        let constant_buffer = TBufferBuilder::<TApi>::new()
            .enable_constant_buffer()
            .with_size(1024)
            .build(device);

        Self {
            view_model,
            command_buffer,
            constant_buffer,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn process_frame(&mut self, queue: &mut TApi::Queue) {
        self.view_model.on_render(queue);

        self.command_buffer.begin();
        self.command_buffer
            .set_constant_buffer(0, &self.constant_buffer);
        self.command_buffer.end();

        queue.flush();
    }
}

pub trait IDesignViewViewModel<TApi: IApi> {
    fn on_render(&mut self, queue: &mut TApi::Queue) {
        queue.flush();
    }

    fn get_camera_position(&self) -> (f32, f32, f32);

    fn get_camera_look_at(&self) -> (f32, f32, f32);

    fn get_camera_up(&self) -> (f32, f32, f32) {
        (0.0, 1.0, 0.0)
    }
}
