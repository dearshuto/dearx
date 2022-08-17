use std::sync::{Arc, Mutex};

use dearx_edit_model::DearxProject;
use dearx_workspace::{DocumentId, Workspace};

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
}
