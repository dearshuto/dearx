use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::egui::FontDefinitions;
use gfx_egui::RenderPass;
use sje_workspace::Workspace;
use sjgfx::{
    api::IApi, TCommandBufferBuilder, TDeviceBuilder, TQueueBuilder, TSemaphoreBuilder,
    TSwapChainBuilder,
};
use sjgfx_interface::{ICommandBuffer, IQueue, ISwapChain, TextureArrayRange};
use winit::event_loop::EventLoop;

struct ExampleRepaintSignal;

impl epi::backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        //self.0.lock().unwrap().send_event(self.).ok();
    }
}

#[tokio::main]
async fn main() {
    run::<sjgfx::api::Wgpu>();
}

fn run<TApi: IApi>() {
    let event_loop = EventLoop::new();
    let mut display = sjvi::create_display(event_loop);

    let mut device =
        TDeviceBuilder::<TApi>::new().build_with_surface(&display.window, &display.event_loop);
    let mut queue = TQueueBuilder::<TApi>::new().build(&device);
    let mut swap_chain = TSwapChainBuilder::<TApi>::new()
        .with_width(2560)
        .with_height(1920)
        .build(&mut device);
    let mut command_buffer = TCommandBufferBuilder::<TApi>::new().build(&device);
    let mut semaphore = TSemaphoreBuilder::<TApi>::new().build(&device);
    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: 1280,
        physical_height: 960,
        scale_factor: display.window.scale_factor(),
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    let _render_pass = RenderPass::<TApi>::new(&device);

    let _workspace = Workspace::<i32>::new();

    while !display.should_close() {
        display.update(|| {
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

            platform.update_time(1.0 / 60.0);
            // platform.begin_frame();
            // let context = platform.context();
            // platform.end_frame(window);

            queue.execute(&command_buffer);
            queue.present(&mut swap_chain);
            queue.sync();
            queue.flush();
        });
    }
}
