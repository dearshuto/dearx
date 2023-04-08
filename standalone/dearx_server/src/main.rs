use std::sync::{Arc, Mutex};

use dearx_application::App;
use dearx_viewer::http::Server;
use dearx_workspace::DocumentInfo;

#[tokio::main]
async fn main() {
    let mut app = App::new();
    app.add_document(&DocumentInfo {
        content: Default::default(),
    });

    let mut server = Server::new(Arc::new(Mutex::new(app)));
    server.serve().await;
}
