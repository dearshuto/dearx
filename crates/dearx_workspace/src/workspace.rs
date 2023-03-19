use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::{document::DocumentId, Document, DocumentInfo, Project};

pub struct Workspace<T: Sync + Send + 'static> {
    pub current_project: Arc<Project<T>>,
    handlers: Arc<Mutex<Vec<Box<dyn FnMut(&Arc<Project<T>>) + Send>>>>,
}

impl<T: Sync + Send + 'static> Workspace<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn observe<F>(&mut self, handler: F) -> Uuid
    where
        F: FnMut(&Arc<Project<T>>) + Send + 'static,
    {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(Box::new(handler));
        let id = Uuid::new_v4();
        id
    }

    pub fn dispose_opservation(&mut self, _id: Uuid) {}

    pub fn add_document(&mut self, document_info: &DocumentInfo<T>) -> DocumentId {
        let new_document = Document::from(document_info);
        let id = DocumentId::new();
        let new_documents = self
            .current_project
            .documents
            .update(id, Arc::new(new_document));
        let new_project = self.current_project.with_documents(new_documents);
        self.current_project = new_project;
        id
    }

    pub fn update_current_project<TFunc: Fn(Arc<T>) -> Arc<T>>(
        &mut self,
        id: &DocumentId,
        updater: TFunc,
    ) {
        if let Some(document) = self.current_project.try_get_document(id) {
            // カレントプロジェクトの更新
            let new_content = updater(document.content.clone());
            let new_document = document.with_content(new_content);
            let new_documts = self
                .current_project
                .documents
                .update(id.clone(), new_document);
            self.current_project = self.current_project.with_documents(new_documts);

            // TODO: 非同期に通知
            let project = self.current_project.clone();
            let handlers_arc = self.handlers.clone();
            let _ = tokio::task::spawn(async move {
                let mut handlers = handlers_arc.lock().unwrap();
                for hander in handlers.iter_mut() {
                    hander(&project);
                }
            });
        }
    }
}

impl<T: Sync + Send + 'static> Default for Workspace<T> {
    fn default() -> Self {
        Self {
            current_project: Arc::new(Project::new()),
            handlers: Default::default(),
        }
    }
}
