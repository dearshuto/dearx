use std::sync::{Arc, Mutex};

use dearx_edit_model::DearxProject;
use dearx_workspace::{DocumentId, Workspace};
use tauri::Manager;

use crate::PropertyWindowViewModel;

pub struct MainWindowViewModel {
    #[allow(dead_code)]
    property_window_view_model: PropertyWindowViewModel,
}

impl MainWindowViewModel {
    pub fn new(id: &DocumentId, workspace: Arc<Mutex<Workspace<DearxProject>>>) -> Self {
        let property_window_view_model = PropertyWindowViewModel::new(id, workspace.clone());

        Self {
            property_window_view_model,
        }
    }

    pub fn listen(&self, app: &tauri::App) {
        app.listen_global("front-to-back", |event| {
            println!("Message from frontend: {:?}", event.payload());
        });

        let app_handle = app.app_handle();
        std::thread::spawn(move || loop {
            app_handle
                .emit_all("back-to-front", "ping frontend".to_string())
                .unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1))
        });

        self.property_window_view_model.listen(app);
    }
}
