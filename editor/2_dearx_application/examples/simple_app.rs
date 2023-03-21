use std::sync::{Arc, Mutex};

use dearx_viewer::http::Server;

#[tokio::main]
async fn main() {
    let app = dearx_application::App::new();
    let app_arc = Arc::new(Mutex::new(app));
    let mut server = Server::new(app_arc);
    server.serve().await;
}
