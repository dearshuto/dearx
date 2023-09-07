// プレビューなしの編集機能

async fn run() {
    let mut client = dearx_viewer::http::Client::default();

    loop {
        if let Ok(color) = client.fetch_color().await {
            println!("{:?}", color);
        } else {
            println!("Connect error");
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[tokio::main]
async fn main() {
    run().await;
}
