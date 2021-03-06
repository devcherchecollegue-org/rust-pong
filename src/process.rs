use crate::interface::{Place, SharedContext};
use tungstenite::Message;

pub struct Process;

impl Process {
    pub fn quit(context: &SharedContext, player_index: usize) {
        match context.lock() {
            Ok(mut context) => context.places[player_index] = Place::Empty,
            _ => (),
        }
    }
    pub fn tick(context: &SharedContext) -> Message {
        match context.lock() {
            Ok(mut context) => match context.places {
                [Place::Empty, _] | [_, Place::Empty] => {
                    Message::Text("Waiting for other player...".into())
                }
                _ => {
                    let data = context.serialize();
                    context.update();
                    Message::Binary(data)
                }
            },
            _ => Message::Text("Packet lost".into()),
        }
    }
}
