use crate::interface::{Place, SharedContext};
use crate::player::Player;
use crate::process::Process;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::Error};

const TICK_RATE: u64 = 1000;

fn fill_places(peer: SocketAddr, context: &SharedContext) -> Result<usize, &str> {
    let player = Player::new(peer);
    match context.lock() {
        Ok(mut context) => match context.places {
            [Place::Empty, _] => {
                context.places[0] = Place::Player(player);
                Ok(0)
            }
            [_, Place::Empty] => {
                context.places[1] = Place::Player(player);
                Ok(1)
            }
            _ => Err("No more place"),
        },
        _ => Err("Mutex locked value"),
    }
}

async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
    context: &SharedContext,
) -> tungstenite::Result<()> {
    log::info!("New WS connection: {}", peer);

    let ws_stream = accept_async(stream).await?;
    let (mut tx, mut rx) = ws_stream.split();
    let mut interval = tokio::time::interval(Duration::from_millis(TICK_RATE));

    loop {
        tokio::select! {
            msg = rx.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() || msg.is_binary() {
                            // TODO: process message
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
                let message = Process::tick(&context);
                tx.send(message).await?;
            }
        }
    }

    Ok(())
}

pub async fn accept_connection(peer: SocketAddr, stream: TcpStream, context: SharedContext) {
    let player_index = match fill_places(peer, &context) {
        Ok(index) => index,
        Err(e) => {
            log::error!("{}", e);
            return;
        }
    };

    if let Err(e) = handle_connection(peer, stream, &context).await {
        Process::quit(&context, player_index);
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => log::error!("Error processing connection: {}", err),
        }
    }
}
