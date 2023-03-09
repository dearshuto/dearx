use dearx_viewer::http::Server;

#[tokio::main]
async fn main() {
    println!("Begin");
    let mut server = Server::default();
    server.serve().await;
    println!("End");
}
