use crate::game::game_message::GameMessage;
use crate::game::team::player::Player;
use crate::position::Position;
use std::any::Any;
use tokio::sync::mpsc::{Receiver, Sender};

pub mod player;

pub struct Team {
    pub players: Vec<Player>,
}

impl Team {
    pub async fn init(&self, game_channel: (&Sender<GameMessage>, &mut Receiver<GameMessage>)) {
        let (_, mut rx) = game_channel;
        while let Some(message) = rx.recv().await {
            println!("Team received message {:?}", message.message_type)
        }
    }
}

pub fn random_team() -> Team {
    let mut players = Vec::new();
    for i in 0..9 {
        match i {
            0 => players.push(Player {
                position: Position::Pitcher,
                number: 32,
            }),
            1 => players.push(Player {
                position: Position::Catcher,
                number: 33,
            }),
            2 => players.push(Player {
                position: Position::FirstBase,
                number: 34,
            }),
            3 => players.push(Player {
                position: Position::SecondBase,
                number: 35,
            }),
            4 => players.push(Player {
                position: Position::ThirdBase,
                number: 36,
            }),
            5 => players.push(Player {
                position: Position::ShortStop,
                number: 37,
            }),
            6 => players.push(Player {
                position: Position::LeftField,
                number: 38,
            }),
            7 => players.push(Player {
                position: Position::CenterField,
                number: 39,
            }),
            8 => players.push(Player {
                position: Position::RightField,
                number: 40,
            }),
            _ => {}
        }
    }
    Team { players }
}
