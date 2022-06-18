use dearx_presentation::{
    DesignView, IDesignViewViewModel, IPropertyWindowViewModel, PropertyWindow,
};
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::egui::FontDefinitions;
use gfx_egui::RenderPass;
use sje_workspace::Workspace;
use sjgfx::{
    api::IApi, TCommandBufferBuilder, TDeviceBuilder, TQueueBuilder, TSemaphoreBuilder,
    TSwapChainBuilder,
};
use sjgfx_interface::{ICommandBuffer, IQueue, ISwapChain, TextureArrayRange};

struct ExampleRepaintSignal;

impl epi::backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        //self.0.lock().unwrap().send_event(self.).ok();
    }
}

struct ViewModel<TApi: sjgfx::api::IApi> {
    _marker: std::marker::PhantomData<TApi>,
}

impl<TApi: sjgfx::api::IApi> ViewModel<TApi> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<TApi: sjgfx::api::IApi> IDesignViewViewModel<TApi> for ViewModel<TApi> {
    fn on_render(&mut self, queue: &mut TApi::Queue) {
        queue.flush();
    }

    fn get_camera_position(&self) -> (f32, f32, f32) {
        todo!()
    }

    fn get_camera_look_at(&self) -> (f32, f32, f32) {
        todo!()
    }

    fn get_camera_up(&self) -> (f32, f32, f32) {
        (0.0, 1.0, 0.0)
    }
}

impl<TApi: sjgfx::api::IApi> IPropertyWindowViewModel for ViewModel<TApi> {
    fn get_translation(&self) -> (f32, f32, f32) {
        (1.0, 2.0, 3.0)
    }
}

#[tokio::main]
async fn main() {
    run::<sjgfx::api::Wgpu>();
}

fn run<TApi: IApi>() {
    let mut instance = sjvi::Instance::new();
    let id = instance.create_display_with_size(2560, 1920);

    let (mut device, mut platform) = {
        let display = instance.try_get_display(id).unwrap();
        let device = TDeviceBuilder::<TApi>::new()
            .build_with_surface(&display.window, instance.get_event_loop());

        let platform = Platform::new(PlatformDescriptor {
            physical_width: 1280,
            physical_height: 960,
            scale_factor: display.window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        (device, platform)
    };
    let mut queue = TQueueBuilder::<TApi>::new().build(&device);
    let mut swap_chain = TSwapChainBuilder::<TApi>::new()
        .with_width(2560)
        .with_height(1920)
        .build(&mut device);
    let mut command_buffer = TCommandBufferBuilder::<TApi>::new().build(&device);
    let mut semaphore = TSemaphoreBuilder::<TApi>::new().build(&device);

    let mut render_pass = RenderPass::<TApi>::new(&device);

    let _workspace = Workspace::<i32>::new();

    let mut design_view = DesignView::<TApi, ViewModel<TApi>>::new(&device, ViewModel::new());
    let mut property_window = PropertyWindow::<ViewModel<TApi>>::new(ViewModel::new());

    while instance.try_update() {
        let display = instance.try_get_display(id).unwrap();

        if display.is_redraw_requested() {
            let mut next_scan_buffer_view =
                swap_chain.acquire_next_scan_buffer_view(Some(&mut semaphore), None);

            command_buffer.begin();
            command_buffer.clear_color(
                &mut next_scan_buffer_view,
                0.0,
                0.2,
                0.3,
                1.0,
                TextureArrayRange::new(),
            );
            command_buffer.end();

            // DesignView
            design_view.process_frame(&mut queue);

            platform.update_time(1.0 / 60.0);
            platform.begin_frame();
            let mut context = platform.context();
            let (_output, paint_commands) =
                context.run(epi::egui::RawInput::default(), |context| {
                    property_window.process_frame(context);
                });
            context.request_repaint();
            platform.end_frame(Some(&display.window));

            let paint_jobs = platform.context().tessellate(paint_commands);
            render_pass.update_buffers(&device, &paint_jobs);
            render_pass.update_texture(&device, &platform.context().font_image());
            render_pass.execute(&next_scan_buffer_view, &mut queue, &paint_jobs);

            queue.execute(&command_buffer);
            queue.present(&mut swap_chain);
            queue.sync();
            queue.flush();
        }

        display.listen(&mut swap_chain);
    }
}
