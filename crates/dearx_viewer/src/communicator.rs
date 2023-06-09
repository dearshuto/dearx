use crate::{
    proto::{GetReply, GetRequest},
    EmbededClient, EmbededServer, IServerLogic,
};

pub trait IClient {
    fn clone(&self) -> Self;

    fn fetch(&mut self, request: &GetRequest) -> Result<GetReply, ()>;
}

pub trait IServer {}

pub struct Communicator<T> {
    _marker: std::marker::PhantomData<T>,
}

// impl Communicator<()> {
//     pub fn create_communicator<T: IServerLogic>(logic: T) -> (EmbededServer<T>, EmbededClient<T>) {
//         let server = EmbededServer::new(logic);
//         let client = server.new_client();
//         (server, client)
//     }
// }
