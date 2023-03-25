use std::collections::HashMap;
use std::sync::Arc;

use dearx_edit_model::{DearxProject, Model, ModelContent};
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

    model_f32_u32_table: HashMap<String, Model<f32, u32>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            workspace: Workspace::new(),
            id_table: Default::default(),
            model_f32_u32_table: Default::default(),
        }
    }

    pub fn add_document(&mut self, document_info: &DocumentInfo<DearxProject>) {
        let id = self.workspace.add_document(document_info);
        self.workspace.update_current_project(&id, |project| {
            project.with_model_contents(Arc::new(vec![
                ModelContent {
                    name: "Trignale".to_string(),
                },
                ModelContent {
                    name: "Cube".to_string(),
                },
            ]))
        });

        // モデルデータを追加
        self.model_f32_u32_table.insert(
            "Triangle".to_string(),
            Model::<f32, u32> {
                vertices: vec![
                    0.0, 0.0, 0.0, // v0
                    0.5, 0.0, 0.0, // v1
                    0.0, 0.5, 0.0, // v2
                ],
                indices: vec![0, 1, 2],
            },
        );
        self.model_f32_u32_table.insert(
            "Cube".to_string(),
            Model::<f32, u32> {
                vertices: vec![
                    1.0, 1.0, -1.0, // v0
                    1.0, -1.0, -1.0, // v1
                    1.0, 1.0, 1.0, // v2
                    1.0, -1.0, -1.0, // v3
                    -1.0, 1.0, -1.0, // v4
                    -1.0, -1.0, 1.0, // v5
                    -1.0, 1.0, 1.0, // v6
                    -1.0, -1.0, 1.0, // v7
                ],
                indices: vec![
                    4, 2, 0, //
                    2, 7, 3, //
                    6, 5, 7, //
                    1, 7, 5, //
                    0, 3, 1, //
                    4, 1, 5, //
                    4, 6, 2, //
                    2, 6, 7, //
                    6, 4, 5, //
                    1, 3, 7, //
                    0, 2, 3, //
                    4, 0, 1, //
                ],
            },
        );

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

            let model_index = if request.mesh_request.is_some() {
                let mesh_request = request.mesh_request.as_ref().unwrap();
                mesh_request.index
            } else {
                0
            };
            let model = self
                .model_f32_u32_table
                .values()
                .nth(model_index as usize)
                .unwrap();

            let vertices = &model.vertices;
            let indices = &model.indices;
            GetReply {
                scene_info_reply: Some(GetSceneInfoReply {
                    red: color[0],
                    green: color[1],
                    blue: color[2],
                    mesh_count: document.content.model_contents.len() as i32,
                }),
                mesh_reply: Some(GetMeshReply {
                    mesh: Some(Mesh {
                        vertices: vertices.clone(),
                        indices: indices.clone(),
                    }),
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
