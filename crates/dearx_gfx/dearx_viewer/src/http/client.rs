use crate::proto::{Color, GetMeshRequest, GetRequest, Mesh};
use prost::Message;
use std::result::Result;

pub struct Shader;

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub async fn fetch_shader(&mut self) -> Result<Shader, ()> {
        let _response = self
            .client
            .get("http://localhost:8080/hello")
            .send()
            .await
            .unwrap();
        Ok(Shader)
    }

    pub async fn fetch_color(&mut self) -> Result<Color, ()> {
        let request = GetRequest {
            ..Default::default()
        };
        let response = self
            .client
            .get("http://localhost:3000/color")
            .query(&request)
            .send()
            .await
            .unwrap();
        let bytes = response.bytes().await.unwrap();
        println!("{}: {:?}", bytes.len(), bytes);
        let color = Color::decode(bytes).unwrap();
        Ok(color)
    }

    pub async fn fetch_mesh(&mut self) -> Result<Mesh, ()> {
        let request = GetMeshRequest {
            ..Default::default()
        };
        let url = "http://localhost:3000/mesh";
        let response = self.client.get(url).query(&request).send().await.unwrap();
        let bytes = response.bytes().await.unwrap();
        let mesh = Mesh::decode(bytes).unwrap();
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
