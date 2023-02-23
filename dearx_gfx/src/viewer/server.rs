use std::sync::{Arc, Mutex};

use super::proto::greeter_server::{Greeter, GreeterServer};
use super::proto::{Reply, ViewerRequest};
use super::IListener;
use tonic::{Request, Response, Status};

pub struct Server<TListener: IListener> {
    listener: Arc<Mutex<TListener>>,
}

impl<TListener: IListener + Send + Sync + 'static> Server<TListener> {
    pub fn new(listener: Arc<Mutex<TListener>>) -> Self {
        Self { listener }
    }

    pub async fn listen(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = "[::1]:50051".parse().unwrap();
        let service = GreeterServer::new(self);
        tonic::transport::Server::builder()
            .add_service(service)
            .serve(addr)
            .await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl<TListener: IListener + Send + Sync + 'static> Greeter for Server<TListener> {
    async fn request(&self, request: Request<ViewerRequest>) -> Result<Response<Reply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        if let Ok(mut listener) = self.listener.lock() {
            listener.on_value_changed()
        }

        let reply = Reply {
            message: format!("Hello {}!", request.into_inner().value),
        };
        Ok(Response::new(reply))
    }
}
