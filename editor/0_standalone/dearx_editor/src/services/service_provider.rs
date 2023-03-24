use super::active_document_manager::ActiveDocumentManager;

#[derive(Debug, Default)]
pub struct ServiceProvider {
    pub active_document_manager: ActiveDocumentManager,
}
