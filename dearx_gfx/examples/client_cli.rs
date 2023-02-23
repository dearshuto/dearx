use dearx_gfx::viewer::{Client, ViewerRequest};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let request = ViewerRequest {
        value: Default::default(),
        sub_request: None,
    };
    client.send(request).await.unwrap();

    println!("Send!");
}
