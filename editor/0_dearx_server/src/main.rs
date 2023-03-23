use std::sync::{Arc, Mutex};

use dearx_application::App;
use dearx_viewer::http::Server;

#[tokio::main]
async fn main() {
    let app = App::new();
    let mut server = Server::new(Arc::new(Mutex::new(app)));
    server.serve().await;
}
