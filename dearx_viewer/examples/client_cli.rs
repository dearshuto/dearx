use dearx_viewer::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let request = dearx_viewer::ViewerRequest {
        value: Default::default(),
        sub_request: None,
    };
    client.send(request).await.unwrap();

    println!("Send!");
}
