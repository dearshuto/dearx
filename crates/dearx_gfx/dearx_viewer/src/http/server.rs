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

use crate::http::server_reply::BinaryRequest;
use crate::proto::{
    CreateReply, CreateRequest, DeleteReply, DeleteRequest, UpdateReply, UpdateRequest,
};

pub trait IServerLogic {
    fn create(&mut self, request: &CreateRequest) -> CreateReply;

    fn delete(&mut self, request: &DeleteRequest) -> DeleteReply;

    fn update(&mut self, request: &UpdateRequest) -> UpdateReply;
}

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
    ) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        Self::get()
            .or(Self::create(logic.clone()))
            .or(Self::delete(logic.clone()))
            .or(Self::update(logic.clone()))
    }

    fn get() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        let color = warp::path!("color")
            .and(warp::get())
            .and_then(Self::get_color_impl);
        let mesh = warp::path!("mesh")
            .and(warp::get())
            .and_then(Self::get_mesh_impl);
        let scene_info = warp::path!("scene_info")
            .and(warp::get())
            .and_then(Self::get_scene_info);
        color.or(mesh).or(scene_info)
    }

    fn create(
        logic: Arc<Mutex<T>>,
    ) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        warp::path!("color")
            .and(warp::post())
            .and(warp::body::bytes())
            .and(Self::with_logic(logic))
            .and_then(Self::create_impl)
    }

    fn update(
        logic: Arc<Mutex<T>>,
    ) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        warp::path!("color")
            .and(warp::put())
            .and(warp::body::bytes())
            .and(Self::with_logic(logic))
            .and_then(Self::update_impl)
    }

    fn delete(
        logic: Arc<Mutex<T>>,
    ) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
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

    async fn get_scene_info() -> Result<impl Reply, warp::Rejection> {
        println!("get resources");
        let mut buffer = Vec::new();
        let reply = crate::proto::GetSceneInfoReply { mesh_count: 0 };

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

    async fn get_color_impl() -> Result<impl Reply, warp::Rejection> {
        println!("get resources");
        let mut buffer = Vec::new();

        let now = chrono::Utc::now();
        let (red, green, blue) = if now.time().second() % 2 == 0 {
            (1.0, 0.0, 0.0)
        } else {
            (0.0, 0.0, 1.0)
        };
        let color = crate::proto::Color {
            red,
            green,
            blue,
            alpha: 0.0,
        };

        color.encode(&mut buffer).unwrap();
        Ok(BinaryRequest::new(buffer))
    }

    async fn get_mesh_impl() -> Result<impl Reply, warp::Rejection> {
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
}

// ロジックなしサーバー。将来的に消すかも
pub struct Empty;
impl IServerLogic for Empty {
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