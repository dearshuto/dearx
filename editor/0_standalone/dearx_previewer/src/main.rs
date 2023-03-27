use std::{
    mem::size_of,
    sync::{Arc, Mutex},
    time::Duration,
};

use dearx_viewer::http::Client;
use wgpu::{util::DeviceExt, VertexAttribute, VertexBufferLayout};
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
    pub index_count: i32,
    pub camera_position: nalgebra::Vector3<f32>,
}

// 通信によって背景色を変える実装
async fn run() {
    let app = Arc::new(Mutex::new(App::default()));
    let app_for_loop = app.clone();
    let mut client = Client::default();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(640, 480))
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
        width: 640,
        height: 480,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    let mesh = client.fetch_mesh().await.unwrap();
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        usage: wgpu::BufferUsages::VERTEX,
        contents: bytemuck::cast_slice(&mesh.vertices),
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        usage: wgpu::BufferUsages::INDEX,
        contents: bytemuck::cast_slice(&mesh.indices),
    });
    app.lock().unwrap().index_count = mesh.indices.len() as i32;

    let shader = client.fetch_shader().await.unwrap();
    println!("Vertex Shader size: {}", shader.vertex_shader_binary.len());
    println!("Pixel Shader size: {}", shader.pixel_shader_binary.len());
    println!(
        "Compute Shader size: {}",
        shader.compute_shader_binary.len()
    );
    let vertex_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::util::make_spirv(&shader.vertex_shader_binary),
    });
    let pixel_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::util::make_spirv(&shader.pixel_shader_binary),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader_module,
            entry_point: "main",
            buffers: &[VertexBufferLayout {
                array_stride: (size_of::<f32>() * 3) as u64, // XYZ
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                }],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &pixel_shader_module,
            entry_point: "main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });
    // 描画用オブジェクトここまで

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

                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(
                        0..app.index_count as u32,
                        0,    /*base vertex*/
                        0..1, /*インスタンス*/
                    );
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
