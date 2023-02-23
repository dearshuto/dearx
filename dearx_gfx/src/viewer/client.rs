use crate::viewer::proto::greeter_client::GreeterClient;
use crate::viewer::proto::ViewerRequest;

pub struct Client;

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn send(&self, request: ViewerRequest) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = GreeterClient::connect("http://[::1]:50051").await?;
        let request = tonic::Request::new(request);

        let response = client.request(request).await?;
        println!("RESPONSE={:?}", response);

        Ok(())
    }
}
