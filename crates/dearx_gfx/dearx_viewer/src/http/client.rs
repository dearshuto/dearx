use crate::proto::{Color, GetReply, GetRequest, Mesh, ShaderBinary};

use prost::Message;
use std::result::Result;

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub async fn fetch(&mut self, _request: &GetRequest) -> Result<GetReply, ()> {
        let response = self
            .client
            .get("http://localhost:3000")
            .send()
            .await
            .unwrap();
        let bytes = response.bytes().await.unwrap();
        if let Ok(reply) = GetReply::decode(bytes) {
            Ok(reply)
        } else {
            Err(())
        }
    }

    pub async fn fetch_shader(&mut self) -> Result<ShaderBinary, ()> {
        let Ok(response) = self.fetch(&Default::default()).await else {
            return Err(());
        };
        let Some(shader_reply) = response.shader_reply else {
            return Err(());
        };
        let Some(shader_binary) = shader_reply.shader_binary else {
            return Err(());
        };

        Ok(shader_binary)
    }

    pub async fn fetch_color(&mut self) -> Result<Color, ()> {
        let Ok(_response) = self.fetch(&Default::default()).await else {
            return Err(());
        };
        Ok(Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        })
    }

    pub async fn fetch_mesh(&mut self) -> Result<Mesh, ()> {
        let Ok(response) = self.fetch(&Default::default()).await else {
            return Err(());
        };
        let Some(mesh_reply) = response.mesh_reply else {
            return Err(());
        };
        let Some(mesh) = mesh_reply.mesh else {
          return Err(());
        };

        Ok(mesh)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client: reqwest::ClientBuilder::new().no_proxy().build().unwrap(),
        }
    }
}
