use dearx_viewer::{IListener, Server};

struct Listener {}

impl IListener for Listener {
    fn on_value_changed(&mut self) {}
}

#[tokio::main]
async fn main() {
    let listener = Listener {};
    let server = Server::new(listener);

    println!("Listen...");
    server.listen().await.unwrap();
}
