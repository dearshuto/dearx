use std::sync::{Arc, Mutex};

use dearx_edit_model::DearxProject;
use dearx_workspace::{DocumentId, Workspace};

use tauri::Manager;

pub struct PropertyWindowViewModel {}

impl PropertyWindowViewModel {
    pub fn new(id: &DocumentId, workspace: Arc<Mutex<Workspace<DearxProject>>>) -> Self {
        let id = id.clone();
        workspace
            .lock()
            .unwrap()
            .observe_project()
            .lock()
            .unwrap()
            .subscribe(move |project| {
                if let Some(document) = project.documents.get(&id) {
                    println!("Value: {}", document.content.value);
                }
            });

        Self {}
    }

    pub fn listen(&self, app: &tauri::App) {
        app.listen_global("input_x_changed", |event| {
            println!("Property Changed: {:?}", event.payload());
        });
    }
}
