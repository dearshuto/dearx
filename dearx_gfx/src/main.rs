use sjvi::IInstance;

use sjgfx::{api::IApi, TDeviceBuilder};

fn main()
{
    if cfg!(target_arch = "wasm32") {
        run::<sjgfx::api::Wasm>();
    } else {
        run::<sjgfx::api::Wgpu>();
    }
}

fn run<TApi: IApi>()
{
    let mut instance = TApi::Instance::new();
    let id = instance.create_display();
    let display = instance.try_get_display(&id).unwrap();
    let _device = TDeviceBuilder::<TApi>::new().enable_debug_assertion().build_with_surface(display);

    while {
        instance.try_update()
    } {}
}
