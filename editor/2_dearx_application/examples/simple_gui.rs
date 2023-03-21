use std::sync::{Arc, Mutex};

use dearx_application::App;
use dearx_viewer::http::Server;
use dearx_workspace::DocumentInfo;
use eframe::egui;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let mut app = App::new();
    app.add_document(&DocumentInfo {
        content: Default::default(),
    });

    let app = Arc::new(Mutex::new(app));
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
}

impl SimpleGui {
    pub fn new(app: Arc<Mutex<App>>) -> Self {
        Self { app }
    }
}

impl eframe::App for SimpleGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut app = self.app.lock().unwrap();
            let project = app.clone_current_project();
            let (id, document) = project.documents.iter().next().unwrap();
            let mut color = document.content.color;
            ui.heading("My egui Application");
            if ui.color_edit_button_rgb(&mut color).changed() {
                app.update_current_project(id, |content| content.with_color(color));
            }
            ui.label(format!("Hello {}, {}, {}", color[0], color[1], color[2]));
        });
    }
}
