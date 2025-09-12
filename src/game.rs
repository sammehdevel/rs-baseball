use crate::game::game_message::{GameMessage, GameMessageType};
use crate::game::team::Team;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

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
    game_channel_tx: Sender<GameMessage>,
    game_channel_rx: Receiver<GameMessage>,
}
impl Game {
    pub async fn start(&mut self) {
        let tx = self.game_channel_tx.clone();
        self.away_team
            .init((&self.game_channel_tx, &mut self.game_channel_rx))
            .await;
        tokio::spawn(async move {
            tx.send(GameMessage {
                message_type: GameMessageType::NewPitch,
            })
            .await
            .unwrap();
        });
    }
}

pub fn init_game(home_team: Team, away_team: Team) -> Game {
    let (tx, mut rx) = mpsc::channel(1);

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
