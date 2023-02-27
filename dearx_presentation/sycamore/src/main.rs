use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use winit::event_loop::EventLoop;

use sycamore_frontend::{js::*, tauri::*};

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        button(on:click=|_| {  }){
            "Button"
        }
    }
}

fn main() {
    // sycamore で生成する UI
    let node = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("sycamore")
        .unwrap();
    sycamore::render_to(App, &node);

    // ボタンにイベントを登録
    let button = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("my_button")
        .unwrap();
    let closure = Closure::wrap(Box::new(move |_e| {
        async move { invoke("cmd", to_value(&GreetArgs { name: "AAA" }).unwrap()).await };
        log("MyLog");
        // alert("message");
    }) as Box<dyn FnMut(JsValue)>);
    button
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();

    // JS 側で破棄してもらうよう Rust 側のメモリ管理から外す
    closure.forget();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    let canvas: HtmlCanvasElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let surface = instance.create_surface_from_canvas(&canvas).unwrap();

    let adapter =
        futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        }))
        .unwrap();

    let (device, queue) = futures::executor::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            limits: wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            features: adapter.features(),
            label: None,
        },
        None,
    ))
    .unwrap();

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: 640,
        height: 480,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    let frame = surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut command_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let _render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
    }
    queue.submit(Some(command_encoder.finish()));
    frame.present();
}
