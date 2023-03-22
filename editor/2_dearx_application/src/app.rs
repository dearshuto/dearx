use std::collections::HashMap;
use std::sync::Arc;

use dearx_edit_model::DearxProject;
use dearx_viewer::http::IServerLogic;
use dearx_viewer::proto::{
    CreateReply, CreateRequest, DeleteReply, DeleteRequest, GetMeshReply, GetReply, GetRequest,
    GetSceneInfoReply, Mesh, UpdateReply, UpdateRequest,
};
use dearx_workspace::{DocumentId, DocumentInfo, Project, Workspace};
use uuid::Uuid;

pub struct App {
    workspace: Workspace<DearxProject>,
    id_table: HashMap<Uuid, DocumentId>,
}

impl App {
    pub fn new() -> Self {
        Self {
            workspace: Workspace::new(),
            id_table: Default::default(),
        }
    }

    pub fn add_document(&mut self, document_info: &DocumentInfo<DearxProject>) {
        let id = self.workspace.add_document(document_info);
        self.id_table.insert(Uuid::new_v4(), id);
    }

    pub fn update_current_project<TFunc: Fn(Arc<DearxProject>) -> Arc<DearxProject>>(
        &mut self,
        id: &DocumentId,
        updater: TFunc,
    ) {
        self.workspace.update_current_project(id, updater);
    }

    pub fn clone_current_project(&self) -> Arc<Project<DearxProject>> {
        self.workspace.current_project.clone()
    }

    fn get_mesh(&self, _request: &GetRequest) -> Option<GetMeshReply> {
        Some(GetMeshReply {
            mesh: Some(Mesh {
                vertices: vec![
                    0.0, 0.0, 0.0, // v0
                    1.0, 0.0, 0.0, // v1
                    0.0, 1.0, 0.0, // v2
                ],
                indices: vec![0, 1, 2],
            }),
        })
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl IServerLogic for App {
    fn get(&mut self, request: &GetRequest) -> GetReply {
        // Workspace から情報を取得
        // TODO: id 引きする
        if let Some(document) = self.workspace.current_project.documents.values().next() {
            let color = document.content.color;
            GetReply {
                scene_info_reply: Some(GetSceneInfoReply {
                    red: color[0],
                    green: color[1],
                    blue: color[2],
                    ..Default::default()
                }),
                mesh_reply: self.get_mesh(request),
                shader_reply: None,
            }
        } else {
            GetReply {
                scene_info_reply: Some(Default::default()),
                mesh_reply: self.get_mesh(request),
                shader_reply: None,
            }
        }
    }

    fn create(&mut self, _request: &CreateRequest) -> CreateReply {
        let document_info = DocumentInfo {
            content: Arc::new(Default::default()),
        };
        let _id = self.workspace.add_document(&document_info);

        CreateReply {
            id: Default::default(),
        }
    }

    fn delete(&mut self, _request: &DeleteRequest) -> DeleteReply {
        DeleteReply {
            id: Default::default(),
        }
    }

    fn update(&mut self, _request: &UpdateRequest) -> UpdateReply {
        UpdateReply {
            id: Default::default(),
        }
    }
}
