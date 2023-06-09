use dearx_application::App;
use dearx_viewer::EmbededServer;
use dearx_workspace::DocumentInfo;
use eframe::egui;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native("", options, Box::new(|cc| Box::new(SimpleGui::new(cc))));
}

#[derive(Default)]
struct SimpleGui {
    #[allow(dead_code)]
    app: Arc<Mutex<App>>,
}

impl SimpleGui {
    pub fn new(context: &eframe::CreationContext) -> Self {
        let state = context.wgpu_render_state.as_ref().unwrap();
        let device = state.device;
        let target_format = state.target_format;

        let scene = {
            let mut factory = dearx_gfx::wgpu::Factory::new(&device, target_format);
            let scene_object = dearx_gfx::serializer::deserialize(&[], &mut factory);
            dearx_gfx::Scene::from_scene_object(scene_object)
        };
        let renderer = dearx_gfx::Renderer::default();

        let app = std::sync::Arc::new(Mutex::new(App::new()));
        let server = EmbededServer::new_shared(app.clone());

        Self { app }
    }

    fn render_custom(&self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

        let callback_backend = egui_wgpu::CallbackFn::new()
            .prepare(move |_, _, _, _| Vec::new())
            .paint(move |_, _, _| ());
        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(callback_backend),
        };
        ui.painter().add(callback);
    }
}

impl eframe::App for SimpleGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World");
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.render_custom(ui);
            });

            // ドキュメントを追加するボタン
            if ui.button("Add").clicked() {
                let content = dearx_edit_model::DearxProject::default();
                let document_info = DocumentInfo {
                    content: Arc::new(content),
                };
                self.app.lock().unwrap().add_document(&document_info);
            }

            let current_project = self.app.lock().unwrap().clone_current_project();

            // ドキュメントの情報を表示
            for (id, document) in &current_project.documents {
                ui.group(|ui| {
                    // ID の表示
                    ui.heading(format!("{:?}", id));

                    // 背景色の表示
                    let current_color = document.content.color;
                    let mut color = current_color.clone();
                    if ui.color_edit_button_rgb(&mut color).changed() {
                        self.app
                            .lock()
                            .unwrap()
                            .update_current_project(&id, |project| project.with_color(color));
                    }

                    for (id, game_object) in document.content.game_object.iter() {
                        ui.label(format!("{:?}", id));

                        for (_id, transform) in game_object.transform_components.iter() {
                            let translation = transform.translation;
                            ui.label(format!(
                                "(x, y, z) = ({}, {}, {})",
                                translation.x, translation.y, translation.z
                            ));
                        }
                    }

                    for model in document.content.model_contents.iter() {
                        ui.label(format!("Name: {}", model.name));
                    }
                });
            }
        });
    }
}
