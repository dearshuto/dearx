mod client;
mod server_reply;

// サーバーは Web では使えない
#[cfg(not(target_arch = "wasm32"))]
mod server;

pub use client::Client;

// サーバーは Web では使えない
#[cfg(not(target_arch = "wasm32"))]
pub use server::Server;
