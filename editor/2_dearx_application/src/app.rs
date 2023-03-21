use std::sync::Arc;

use dearx_viewer::http::IServerLogic;
use dearx_viewer::proto::{
    CreateReply, CreateRequest, DeleteReply, DeleteRequest, GetMeshReply, GetReply, GetRequest,
    GetSceneInfoReply, Mesh, UpdateReply, UpdateRequest,
};
use dearx_workspace::{DocumentInfo, Workspace};

#[derive(Default)]
pub struct DocumentData;

pub struct App {
    workspace: Workspace<DocumentData>,
    pub color: [f32; 3],
}

impl App {
    pub fn new() -> Self {
        Self {
            workspace: Workspace::new(),
            color: [1.0, 1.0, 1.0],
        }
    }

    pub fn add_document(&mut self, document_info: &DocumentInfo<DocumentData>) {
        self.workspace.add_document(document_info);
    }

    fn get_mesh(&self, request: &GetRequest) -> Option<GetMeshReply> {
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
        GetReply {
            scene_info_reply: Some(GetSceneInfoReply {
                red: self.color[0],
                green: self.color[1],
                blue: self.color[2],
                ..Default::default()
            }),
            mesh_reply: self.get_mesh(request),
            shader_reply: None,
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
