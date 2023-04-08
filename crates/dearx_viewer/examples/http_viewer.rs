use std::{thread::sleep, time::Duration};

use tokio::sync::watch::Sender;
use winit::window::Window;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
};

use dearx_viewer::http;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct Client {
    #[allow(dead_code)]
    client: http::Client,
    sender: Sender<wgpu::Color>,
    window: Window,
}

impl Client {
    pub fn new(sender: Sender<wgpu::Color>, window: Window) -> Self {
        Self {
            client: Default::default(),
            sender,
            window,
        }
    }

    pub async fn listen(&mut self) -> bool {
        let color = self.client.fetch_color().await.unwrap();
        println!("({}, {}, {})", color.red, color.green, color.blue);
        let color = wgpu::Color {
            r: color.red as f64,
            g: color.green as f64,
            b: color.blue as f64,
            a: 1.0,
        };
        self.sender.send(color).unwrap();
        self.window.request_redraw();
        true
    }
}

#[cfg(not(target = "wasm"))]
#[tokio::main]
async fn main() {
    let mesh = dearx_viewer::http::Client::default()
        .fetch_mesh()
        .await
        .unwrap();
    println!("{:?}", mesh.vertices);
    println!("{:?}", mesh.indices);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(640, 480))
        .build(&event_loop)
        .unwrap();

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

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

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

    let (sender, reciever) = tokio::sync::watch::channel(wgpu::Color::BLACK);
    let mut client = Client::new(sender, window);
    let _handle = tokio::task::spawn(async move {
        while client.listen().await {
            sleep(Duration::from_millis(1000_u64));
        }
    });

    event_loop.run(move |event, _, control_flow| {
        let _ = (&instance, &adapter, &pipeline_layout);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                let color = *reciever.borrow();

                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(color),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });
                }
                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

// wasm not supported
#[cfg(target = "wasm")]
fn main() {}
