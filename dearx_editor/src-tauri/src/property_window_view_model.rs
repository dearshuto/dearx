use std::sync::{Arc, Mutex};

use dearx_edit_model::DearxProject;
use dearx_workspace::{Observable, Workspace};

use crate::ServiceProvider;
use serde_json::Value;
use tauri::Manager;

pub struct PropertyWindowViewModel {
    workspace: Arc<Mutex<Workspace<DearxProject, ServiceProvider>>>,
    subscription: Option<Arc<Mutex<Observable<DearxProject>>>>,
}

impl PropertyWindowViewModel {
    pub fn new(workspace: Arc<Mutex<Workspace<DearxProject, ServiceProvider>>>) -> Self {
        Self {
            workspace,
            subscription: None,
        }
    }

    pub fn listen(&mut self, app: &tauri::App) {
        // Model -> View
        let working_workspace = self.workspace.clone();
        let app_handle = app.app_handle();
        if let Ok(mut workspace) = self.workspace.lock() {
            let observable = workspace.observe_project();
            if let Ok(mut observable_locked) = observable.lock() {
                observable_locked.subscribe(move |project| {
                    if let Ok(workspace) = working_workspace.lock() {
                        if let Some(id) =
                            workspace.mutable_instance.active_document_manager.active_id
                        {
                            if let Some(_document) = project.try_get_document(&id) {
                                app_handle
                                    .emit_all("property_changed", "test".to_string())
                                    .unwrap();
                            }
                        }
                    }
                });
            }
            self.subscription = Some(observable);
        }

        // View -> Model
        let workspace = self.workspace.clone();
        app.listen_global("input_x_changed", move |event| {
            // 編集対象の id を検索
            let id = if let Ok(workspace) = workspace.lock() {
                workspace.mutable_instance.active_document_manager.active_id
            } else {
                None
            };

            if let Some(id) = id {
                if let Ok(mut workspace) = workspace.lock() {
                    workspace.update_current_project(&id, |x| {
                        let v: Value = serde_json::from_str(event.payload().unwrap()).unwrap();
                        let value = &v["value"];
                        println!("Property Changed: {}", value.as_f64().unwrap());
                        x // とりあえずそのまま返す
                    });
                }
            }
        });
    }
}
