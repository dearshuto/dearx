use crate::proto::greeter_client::GreeterClient;

pub struct Client;

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn send(
        &self,
        request: crate::proto::ViewerRequest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = GreeterClient::connect("http://[::1]:50051").await?;
        let request = tonic::Request::new(request);

        let response = client.request(request).await?;
        println!("RESPONSE={:?}", response);

        Ok(())
    }
}
