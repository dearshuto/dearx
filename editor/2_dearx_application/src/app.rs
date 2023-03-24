use std::collections::HashMap;
use std::sync::Arc;

use dearx_edit_model::DearxProject;
use dearx_viewer::proto::{
    CreateReply, CreateRequest, DeleteReply, DeleteRequest, GetMeshReply, GetReply, GetRequest,
    GetSceneInfoReply, GetShaderReply, Mesh, ShaderBinary, UpdateReply, UpdateRequest,
};
use dearx_viewer::IServerLogic;
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
        self.workspace.update_current_project(&id, |project| {
            project
                .with_vertives(vec![
                    0.0, 0.0, 0.0, // v0
                    0.5, 0.0, 0.0, // v1
                    0.0, 0.5, 0.0, // v2
                ])
                .with_indices(vec![0, 1, 2])
        });
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
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl IServerLogic for App {
    fn get(&mut self, request: &GetRequest) -> GetReply {
        let vertex_shader_source =
            include_str!("../../../crates/dearx_gfx/dearx_gfx/resources/shaders/triangle.vs");
        let pixel_shader_source =
            include_str!("../../../crates/dearx_gfx/dearx_gfx/resources/shaders/triangle.fs");
        let mut compiler = sjgfx_util::ShaderCompiler::new();
        let vertex_shader_binary =
            compiler.create_binary(vertex_shader_source, sjgfx_util::ShaderStage::Vertex);
        let pixel_shader_binary =
            compiler.create_binary(pixel_shader_source, sjgfx_util::ShaderStage::Pixel);
        let shader_binary = Some(ShaderBinary {
            vertex_shader_binary,
            pixel_shader_binary,
            compute_shader_binary: Vec::new(),
        });

        // Workspace から情報を取得
        // TODO: id 引きする
        if let Some(document) = self.workspace.current_project.documents.values().next() {
            let color = document.content.color;
            let vertices = document.content.vertives.clone();
            let indices = document.content.indices.clone();
            GetReply {
                scene_info_reply: Some(GetSceneInfoReply {
                    red: color[0],
                    green: color[1],
                    blue: color[2],
                    ..Default::default()
                }),
                mesh_reply: Some(GetMeshReply {
                    mesh: Some(Mesh { vertices, indices }),
                }),
                shader_reply: Some(GetShaderReply { shader_binary }),
            }
        } else {
            GetReply {
                scene_info_reply: Some(Default::default()),
                mesh_reply: None,
                shader_reply: Some(GetShaderReply { shader_binary }),
            }
        }
    }

    fn create(&mut self, _request: &CreateRequest) -> CreateReply {
        Default::default()
    }

    fn delete(&mut self, _request: &DeleteRequest) -> DeleteReply {
        Default::default()
    }

    fn update(&mut self, _request: &UpdateRequest) -> UpdateReply {
        Default::default()
    }
}
