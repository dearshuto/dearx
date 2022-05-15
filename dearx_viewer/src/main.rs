use dearx_gfx::{Renderer, Scene};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().build(&event_loop);

    let mut scene = Scene::new();
    let renderer = Renderer::new();

    event_loop.run(move |_, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        scene.update();
        renderer.render(&scene);
    });
}
