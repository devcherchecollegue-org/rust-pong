mod ball;
mod connection;
mod interface;
mod logger;
mod player;
mod process;
use interface::Context;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    logger::init();

    let addr = std::env::var("SERVER_ADDR").expect("SERVER_ADDR should be set in .env");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Can't listen");
    let context = Arc::new(Mutex::new(Context::new()));

    log::info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        match stream.peer_addr() {
            Ok(peer) => {
                let context = Arc::clone(&context);
                tokio::spawn(connection::accept_connection(peer, stream, context));
            }
            Err(e) => log::error!("{}", e),
        }
    }
}
