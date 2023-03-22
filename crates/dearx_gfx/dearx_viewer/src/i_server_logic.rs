use crate::proto::{
    CreateReply, CreateRequest, DeleteReply, DeleteRequest, GetReply, GetRequest, UpdateReply,
    UpdateRequest,
};

pub trait IServerLogic {
    fn get(&mut self, request: &GetRequest) -> GetReply;

    fn create(&mut self, request: &CreateRequest) -> CreateReply;

    fn delete(&mut self, request: &DeleteRequest) -> DeleteReply;

    fn update(&mut self, request: &UpdateRequest) -> UpdateReply;
}
