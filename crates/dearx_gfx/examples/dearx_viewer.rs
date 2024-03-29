// use dearx_gfx::{Renderer, Scene, SceneUpdater};
use sjgfx::api::IApi;

fn main() {
    run::<sjgfx::api::Wgpu>();
}

fn run<TApi: IApi>() {
    // let mut instance = sjvi::Instance::new();
    // let id = instance.create_display_with_size(1280, 960);

    // let mut device = {
    //     let display = instance.try_get_display(id).unwrap();
    //     TDeviceBuilder::<TApi>::new()
    //         .enable_debug_assertion()
    //         .build_with_surface(&display)
    // };
    // let mut queue = TQueueBuilder::<TApi>::new().build(&mut device);
    // let mut swap_chain = TSwapChainBuilder::<TApi>::new()
    //     .with_width(1280)
    //     .with_height(960)
    //     .build(&mut device);
    // let mut command_buffer = TCommandBufferBuilder::<TApi>::new().build(&device);
    // let mut semaphore = TSemaphoreBuilder::<TApi>::new().build(&device);

    // let mut scene = Scene::<TApi>::new(&mut device);
    // let mut renderer = Renderer::<TApi>::new(&mut device);

    // while instance.try_update() {
    //     let display = instance.try_get_display(id).unwrap();
    //     if display.is_redraw_requested() {
    //         let scene_updater = SceneUpdater::new();
    //         scene_updater.update(&mut scene);

    //         let mut color_target_view =
    //             swap_chain.acquire_next_scan_buffer_view(Some(&mut semaphore), None);
    //         command_buffer.begin();
    //         command_buffer.clear_color(
    //             &mut color_target_view,
    //             0.0,
    //             0.0,
    //             0.1,
    //             0.0,
    //             TextureArrayRange::new(),
    //         );
    //         command_buffer.end();
    //         queue.execute(&command_buffer);

    //         renderer.make_command(&scene, &color_target_view);

    //         for command_buffer in renderer.get_command_buffers() {
    //             queue.execute(command_buffer);
    //         }

    //         queue.present(&mut swap_chain);
    //         queue.flush();
    //         queue.sync();
    //     }

    //     display.listen(&mut swap_chain);
    // }
}
