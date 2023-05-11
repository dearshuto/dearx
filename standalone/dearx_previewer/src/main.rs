use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use dearx_gfx::{
    serializer::deserialize,
    wgpu::{Factory, Scene},
    Renderer,
};
use dearx_viewer::http::Client;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// とりあえず通信結果を保持するためだけのオブジェクト
// 理想は Workspace から情報をとってきたい
#[derive(Default)]
struct App {
    pub color: [f32; 3],
}

// 通信によって背景色を変える実装
async fn run() {
    let app = Arc::new(Mutex::new(App::default()));
    let app_for_loop = app.clone();
    let mut client = Client::default();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1280, 960))
        .build(&event_loop)
        .unwrap();

    // 描画用オブジェクト
    let instance = wgpu::Instance::default();
    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: 1280,
        height: 960,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    let _mesh = client.fetch_mesh().await.unwrap();

    let shader = client.fetch_shader().await.unwrap();
    println!("Vertex Shader size: {}", shader.vertex_shader_binary.len());
    println!("Pixel Shader size: {}", shader.pixel_shader_binary.len());
    println!(
        "Compute Shader size: {}",
        shader.compute_shader_binary.len()
    );

    // サーバーからの情報を取得
    let (sender, reciever) = std::sync::mpsc::channel::<bool>();
    let _task = tokio::task::spawn(async move {
        loop {
            // 20fps で同期
            std::thread::sleep(Duration::from_millis(250));

            let color = client.fetch_color().await.unwrap();
            let mut app = app.lock().unwrap();
            app.color = [color.red, color.green, color.blue];
            // println!("{}, {}, {}", color.red, color.green, color.blue);
            window.request_redraw();

            match reciever.try_recv() {
                Ok(_) => break,
                Err(_) => continue,
            };
        }
    });

    let scene = {
        let mut factory = Factory::new(&device, swapchain_format);
        let scene_object = deserialize(&[], &mut factory);
        Scene::new_graphics(&device, swapchain_format, scene_object)
    };
    let renderer = Renderer::default();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                let app = app_for_loop.lock().unwrap();
                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: app.color[0] as f64,
                                    g: app.color[1] as f64,
                                    b: app.color[2] as f64,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });

                    renderer.render(&mut render_pass, &scene, scene.enumerate_draw_info());
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                sender.send(true).unwrap();
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

#[tokio::main]
async fn main() {
    run().await;
}
