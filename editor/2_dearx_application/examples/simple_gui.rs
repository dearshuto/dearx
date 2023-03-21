use std::sync::{Arc, Mutex};

use dearx_application::App;
use dearx_viewer::http::Server;
use eframe::egui;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let app = Arc::new(Mutex::new(App::new()));
    let app_for_server = app.clone();
    let task = tokio::spawn(async move {
        let mut server = Server::new(app_for_server);
        server.serve().await;
    });

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(SimpleGui::new(app))),
    )
    .unwrap();
    task.await.unwrap();
}

#[derive(Default)]
struct SimpleGui {
    #[allow(dead_code)]
    app: Arc<Mutex<App>>,
    color: [f32; 3],
}

impl SimpleGui {
    pub fn new(app: Arc<Mutex<App>>) -> Self {
        Self {
            app,
            color: [1.0, 1.0, 1.0],
        }
    }
}

impl eframe::App for SimpleGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut app = self.app.lock().unwrap();
            let mut color = app.color;
            ui.heading("My egui Application");
            if ui.color_edit_button_rgb(&mut color).changed() {
                app.color = color;
            }
            ui.label(format!(
                "Hello {}, {}, {}",
                self.color[0], self.color[1], self.color[2]
            ));
        });
    }
}
