use std::sync::{Arc, Mutex};

use dearx_edit_model::DearxProject;
use dearx_workspace::Workspace;

// use tauri::Manager;

use crate::ServiceProvider;

pub struct ObjectTreeViewModel;

impl ObjectTreeViewModel {
    pub fn new(_workspace: Arc<Mutex<Workspace<DearxProject, ServiceProvider>>>) -> Self {
        Self {}
    }

    // pub fn listen(&self, app: &tauri::App) {
    //     app.listen_global("selection_changed", |event| {
    //         println!("selection_changed: {:?}", event.payload());
    //     });
    // }
}
