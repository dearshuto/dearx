use std::sync::{Arc, Mutex};

use dearx_edit_model::DearxProject;
use dearx_workspace::Workspace;
use tauri::Manager;

use crate::PropertyWindowViewModel;

pub struct MainWindowViewModel {
    #[allow(dead_code)]
    property_window_view_model: PropertyWindowViewModel,
}

impl MainWindowViewModel {
    pub fn new(
        workspace: Arc<Mutex<Workspace<DearxProject>>>,
    ) -> Self {
        let property_window_view_model =
            PropertyWindowViewModel::new(workspace.clone());

        Self {
            property_window_view_model,
        }
    }

    pub fn listen(&self, app: &tauri::App) {
        app.listen_global("front-to-back", |event| {
            println!("Message from frontend: {:?}", event.payload());
        });
    }
}
