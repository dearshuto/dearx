use dearx_workspace::DocumentId;

#[derive(Debug, Default)]
pub struct ActiveDocumentManager {
    pub active_id: Option<DocumentId>,
}
