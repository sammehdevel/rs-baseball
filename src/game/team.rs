use crate::game::game_message::{GameMessage, GameMessageType};
use crate::game::team::player::Player;
use crate::position::Position;
use rand::{rng, Rng};
use tokio::sync::broadcast::{Receiver, Sender};

pub mod player;

pub struct Team {
    pub players: Vec<Player>,
}

impl Team {
    pub async fn init(&mut self) {}
    pub fn start_pitching(
        &mut self,
        mut game_channel: (Sender<GameMessage>, Receiver<GameMessage>),
    ) {
        tokio::spawn(async move {
            loop {
                let message = game_channel.1.recv().await;
                if message.is_err() {
                    println!("err::{}", message.err().unwrap())
                } else {
                    match message.unwrap().message_type {
                        GameMessageType::NewPitch => {
                            if rng().random::<f64>() < 0.2 {
                                game_channel
                                    .0
                                    .send(GameMessage {
                                        message_type: GameMessageType::Strike,
                                    })
                                    .expect("Could not send message from team");
                            } else {
                                game_channel
                                    .0
                                    .send(GameMessage {
                                        message_type: GameMessageType::Ball,
                                    })
                                    .expect("Could not send message from team");
                            }
                        }

                        _ => {}
                    }
                }
            }
        });
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
