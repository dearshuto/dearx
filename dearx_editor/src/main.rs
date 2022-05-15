use eframe::egui;

struct DearxEditor;

impl Default for DearxEditor {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for DearxEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            // スライダーでモデルを編集
            // let current_project = self.workspace.current_project.clone();
            // for (id, document) in &current_project.documents {
            //     let mut value = document.content.value;
            //     ui.add(egui::Slider::new(&mut value, 0..=10).text("Value"));

            //     if document.content.value != value {
            //         self.workspace
            //             .update_current_project(&id, |x| x.with_value(value));
            //     }
            // }

            // 編集結果をラベルに表示
            // for (_id, document) in &current_project.documents {
            //     ui.label(format!("Hello World: {}", document.content.value));
            // }
        });

        egui::Window::new("My Window").show(ctx, |ui| {
            ui.label("Hello World!");
        });
    }
}

#[tokio::main]
async fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(DearxEditor::default())),
    );
}
