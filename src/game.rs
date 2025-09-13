use crate::game::game_message::{GameMessage, GameMessageType};
use crate::game::team::Team;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::position::Position::Pitcher;

mod game_message;
pub mod team;

pub struct Game {
    pub half_inning: u8,
    pub strikes: u8,
    pub balls: u8,
    pub outs: u8,
    pub runner_on_first: bool,
    pub runner_on_second: bool,
    pub runner_on_third: bool,
    pub home_team: Team,
    pub away_team: Team,
    pub game_channel_tx: Sender<GameMessage>,
    pub game_channel_rx: Receiver<GameMessage>,
}

impl Game {
    pub async fn start(&mut self) {
        let tx = self.game_channel_tx.clone();
        let rx = self.game_channel_tx.subscribe();
        self.home_team.start_pitching((tx, rx));
        self.send_next_pitch();
        loop {
            let message = self.game_channel_rx.recv().await.unwrap();
            match message.message_type {
                GameMessageType::Strike => {
                    self.strikes += 1;
                    if self.strikes == 3 {
                        self.reset_count();
                        self.send_out();
                    } else {
                        self.send_next_pitch();
                    }
                }
                GameMessageType::Ball => {
                    self.balls += 1;
                    if self.balls == 4 {
                        self.reset_count();
                        self.send_walk();
                    } else {
                        self.send_next_pitch();
                    }
                }
                GameMessageType::Out => {
                    self.outs += 1;
                    if self.outs == 3 {
                        break;
                    } else {
                        self.send_next_pitch();
                    }
                }
                GameMessageType::Walk => {
                    if !self.runner_on_first {
                        self.runner_on_first = true;
                    } else if !self.runner_on_second {
                        self.runner_on_second = true;
                    } else if !self.runner_on_third {
                        self.runner_on_third = true;
                    } else {
                        println!("Run Scored!!");
                    }
                    self.send_next_pitch();
                }
                _ => {}
            }
        }
    }

    fn reset_count(&mut self) {
        self.strikes = 0;
        self.balls = 0
    }

    fn send_next_pitch(&self) {
        self.game_channel_tx
            .send(GameMessage {
                message_type: GameMessageType::NewPitch,
            })
            .unwrap();
    }
    fn send_out(&self) {
        self.game_channel_tx
            .send(GameMessage {
                message_type: GameMessageType::Out,
            })
            .unwrap();
    }
    fn send_walk(&self) {
        self.game_channel_tx
            .send(GameMessage {
                message_type: GameMessageType::Walk,
            })
            .unwrap();
    }
}

pub async fn init_game(home_team: Team, away_team: Team) -> Game {
    let (tx, rx) = broadcast::channel(10);
    Game {
        half_inning: 0,
        strikes: 0,
        balls: 0,
        outs: 0,
        runner_on_first: false,
        runner_on_second: false,
        runner_on_third: false,
        home_team,
        away_team,
        game_channel_tx: tx,
        game_channel_rx: rx,
    }
}
