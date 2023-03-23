use chrono::Timelike;
use hyper::StatusCode;
use prost::Message;
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
// #![deny(warnings)]
#[cfg(not(target_arch = "wasm32"))]
use warp::Filter;
use warp::Reply;

use crate::proto::{
    CreateReply, CreateRequest, DeleteReply, DeleteRequest, GetMeshRequest, GetReply, GetRequest,
    GetSceneInfoRequest, GetShaderRequest, UpdateReply, UpdateRequest,
};
use crate::{http::server_reply::BinaryRequest, IServerLogic};

pub struct Server<T: Send + IServerLogic> {
    value: Arc<Mutex<T>>,
}

impl<T: Send + IServerLogic + 'static> Server<T> {
    pub fn new(value: Arc<Mutex<T>>) -> Self {
        Self { value }
    }

    pub async fn serve(&mut self) {
        let api = Self::api(self.value.clone());
        warp::serve(api).run(([0, 0, 0, 0], 3000)).await;
    }

    pub fn api(
        logic: Arc<Mutex<T>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        Self::get(logic.clone())
            .or(Self::create(logic.clone()))
            .or(Self::delete(logic.clone()))
            .or(Self::update(logic))
    }

    fn get(
        logic: Arc<Mutex<T>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let color = warp::path("color")
            .and(warp::query::<GetRequest>())
            .and(Self::with_logic(logic.clone()))
            .and_then(Self::get_color_impl);
        let mesh = warp::path("mesh")
            .and(warp::query::<GetMeshRequest>())
            .and_then(Self::get_mesh_impl);
        let scene_info = warp::path("scene_info")
            .and(warp::query::<GetSceneInfoRequest>())
            .and_then(Self::get_scene_info);
        let shader = warp::path("shader")
            .and(warp::query::<GetShaderRequest>())
            .and(Self::with_logic(logic))
            .and_then(Self::get_shader_impl);

        color.or(mesh).or(scene_info).or(shader)
    }

    fn create(
        logic: Arc<Mutex<T>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("color")
            .and(warp::post())
            .and(warp::body::bytes())
            .and(Self::with_logic(logic))
            .and_then(Self::create_impl)
    }

    fn update(
        logic: Arc<Mutex<T>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("color")
            .and(warp::put())
            .and(warp::body::bytes())
            .and(Self::with_logic(logic))
            .and_then(Self::update_impl)
    }

    fn delete(
        logic: Arc<Mutex<T>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("color")
            .and(warp::delete())
            .and(warp::body::bytes())
            .and(Self::with_logic(logic))
            .and_then(Self::delete_impl)
    }

    fn with_logic(
        logic: Arc<Mutex<T>>,
    ) -> impl Filter<Extract = (Arc<Mutex<T>>,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || logic.clone())
    }

    async fn get_scene_info(_: GetSceneInfoRequest) -> Result<impl Reply, warp::Rejection> {
        println!("get resources");
        let mut buffer = Vec::new();
        let reply = crate::proto::GetSceneInfoReply {
            mesh_count: 0,
            ..Default::default()
        };

        reply.encode(&mut buffer).unwrap();
        Ok(BinaryRequest::new(buffer))
    }

    async fn create_impl(
        byte: ::bytes::Bytes,
        logic: Arc<Mutex<T>>,
    ) -> Result<impl Reply, Infallible> {
        let request = CreateRequest::decode(byte).unwrap();
        logic.lock().unwrap().create(&request);
        Ok(StatusCode::NO_CONTENT)
    }

    async fn delete_impl(
        byte: ::bytes::Bytes,
        logic: Arc<Mutex<T>>,
    ) -> Result<impl Reply, Infallible> {
        let request = DeleteRequest::decode(byte).unwrap();
        logic.lock().unwrap().delete(&request);
        Ok(StatusCode::NO_CONTENT)
    }

    async fn update_impl(
        byte: ::bytes::Bytes,
        logic: Arc<Mutex<T>>,
    ) -> Result<impl Reply, Infallible> {
        let request = UpdateRequest::decode(byte).unwrap();
        logic.lock().unwrap().update(&request);
        Ok(StatusCode::NO_CONTENT)
    }

    async fn get_color_impl(
        request: GetRequest,
        logic: Arc<Mutex<T>>,
    ) -> Result<impl Reply, warp::Rejection> {
        let mut logic = logic.lock().unwrap();
        let reply = logic.get(&request);

        // デバッグ目的で常にカラーを返したいので、なにかしらの値を返しておく
        // ロジックがリクエストを返したらそちらを採用
        let (red, green, blue) = if let Some(scene_info) = reply.scene_info_reply {
            let red = scene_info.red;
            let green = scene_info.green;
            let blue = scene_info.blue;
            (red, green, blue)
        } else {
            let now = chrono::Utc::now();
            if now.time().second() % 2 == 0 {
                (1.0, 0.0, 0.0)
            } else {
                (0.0, 0.0, 1.0)
            }
        };

        let color = crate::proto::Color {
            red,
            green,
            blue,
            alpha: 0.0,
        };

        // バイナリ化
        let mut buffer = Vec::new();
        color.encode(&mut buffer).unwrap();

        Ok(BinaryRequest::new(buffer))
    }

    async fn get_mesh_impl(_: GetMeshRequest) -> Result<impl Reply, warp::Rejection> {
        println!("get resources");
        let mut buffer = Vec::new();

        let mesh = crate::proto::Mesh {
            vertices: vec![
                0.0, 0.0, 0.0, // v0
                1.0, 0.0, 0.0, // v1
                0.0, 1.0, 0.0, // v2
            ],
            indices: vec![0, 1, 2],
        };

        mesh.encode(&mut buffer).unwrap();
        Ok(BinaryRequest::new(buffer))
    }

    async fn get_shader_impl(
        request: GetShaderRequest,
        logic: Arc<Mutex<T>>,
    ) -> Result<impl Reply, warp::Rejection> {
        let mut logic = logic.lock().unwrap();
        let reply = logic.get(&GetRequest {
            scene_info_request: None,
            mesh_request: None,
            shader_request: Some(request),
        });

        let shader_binary = if let Some(shader_binary) = reply.shader_reply {
            if let Some(binary) = shader_binary.shader_binary {
                crate::proto::ShaderBinary {
                    vertex_shader_binary: binary.vertex_shader_binary,
                    pixel_shader_binary: binary.pixel_shader_binary,
                    compute_shader_binary: binary.compute_shader_binary,
                }
            } else {
                Default::default()
            }
        } else {
            Default::default()
        };
        let mut buffer = Vec::new();
        shader_binary.encode(&mut buffer).unwrap();
        Ok(BinaryRequest::new(buffer))
    }
}

// ロジックなしサーバー。将来的に消すかも
pub struct Empty;
impl IServerLogic for Empty {
    fn get(&mut self, _request: &GetRequest) -> GetReply {
        Default::default()
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

impl Default for Server<Empty> {
    fn default() -> Self {
        Self {
            value: Arc::new(Mutex::new(Empty {})),
        }
    }
}
