use std::sync::{Arc, Mutex};

use crate::{IClient, IServer, IServerLogic};

pub struct EmbededServer<T> {
    logic: T,
}

impl<TLogic: IServerLogic> EmbededServer<TLogic> {
    // pub fn new(logic: TLogic) -> Self {
    //     Self {
    //         logic: Arc::new(Mutex::new(logic)),
    //     }
    // }
}

impl<TLogic: IServerLogic> EmbededServer<Arc<Mutex<TLogic>>> {
    pub fn new_shared(logic: Arc<Mutex<TLogic>>) -> Self {
        Self { logic }
    }

    pub fn new_client(&self) -> EmbededClient<TLogic> {
        EmbededClient {
            server_logic: self.logic.clone(),
        }
    }
}

impl<T: IServerLogic> IServer for EmbededServer<T> {}

pub struct EmbededClient<TServerLogic: IServerLogic> {
    server_logic: Arc<Mutex<TServerLogic>>,
}

impl<TServerLogic: IServerLogic> IClient for EmbededClient<TServerLogic> {
    fn clone(&self) -> Self {
        Self {
            server_logic: self.server_logic.clone(),
        }
    }

    fn fetch(&mut self, request: &crate::proto::GetRequest) -> Result<crate::proto::GetReply, ()> {
        let reply = self.server_logic.lock().unwrap().get(request);
        Ok(reply)
    }
}
