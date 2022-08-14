use std::sync::{Arc, Mutex};

use dearx_edit_model::DearxProject;
use dearx_workspace::Workspace;

pub struct PropertyWindowViewModel {}

impl PropertyWindowViewModel {
    pub fn new(
        _workspace: Arc<Mutex<Workspace<DearxProject>>>,
    ) -> Self {
        Self {}
    }
}
