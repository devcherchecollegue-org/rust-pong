use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio_tungstenite::connect_async;
use tungstenite::{Message, Result};

const TICK_RATE: u64 = 1000;

struct Player {
    y: f32,
    vy: f32,
}

impl Player {
    pub fn new() -> Self {
        Self { y: 0.0, vy: 0.0 }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let y = self.y.to_le_bytes();
        let vy = self.vy.to_le_bytes();
        vec![y[0], y[1], y[2], y[3], vy[0], vy[1], vy[2], vy[3]]
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = url::Url::parse("ws://127.0.0.1:9002").unwrap();
    let (ws_stream, _) = connect_async(url).await.unwrap();
    let (mut tx, mut rx) = ws_stream.split();
    let mut interval = tokio::time::interval(Duration::from_millis(TICK_RATE));

    let player = Player::new();

    loop {
        tokio::select! {
            msg = rx.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() || msg.is_binary() {
                            println!("{:?}", msg);
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            _ = interval.tick() => tx.send(Message::Binary(player.serialize())).await?,
        }
    }
    Ok(())
}
