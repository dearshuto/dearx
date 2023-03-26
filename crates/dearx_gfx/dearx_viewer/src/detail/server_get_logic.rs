use chrono::Timelike;
use std::sync::{Arc, Mutex};

use crate::proto::{Color, GetMeshReply, GetMeshRequest, GetRequest, Mesh};
use crate::IServerLogic;

pub struct ServerGetLogic<T>
where
    T: Send + IServerLogic + 'static,
{
    _marker: std::marker::PhantomData<T>,
}

impl<T> ServerGetLogic<T>
where
    T: Send + IServerLogic + 'static,
{
    pub fn get_color(request: GetRequest, logic: Arc<Mutex<T>>) -> Result<Color, ()> {
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

        Ok(crate::proto::Color {
            red,
            green,
            blue,
            alpha: 0.0,
        })
    }

    #[allow(dead_code)]
    async fn get_mesh(request: GetMeshRequest, logic: Arc<Mutex<T>>) -> Result<GetMeshReply, ()> {
        let mut logic = logic.lock().unwrap();
        let reply = logic.get(&GetRequest {
            mesh_request: Some(request),
            ..Default::default()
        });

        let Some(mesh_reply) = reply.mesh_reply else {
            return Err(());
        };

        Ok(mesh_reply)
    }
}
