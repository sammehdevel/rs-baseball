#[derive(Debug)]
pub struct GameMessage {
    pub message_type: GameMessageType,
}

impl Clone for GameMessage {
    fn clone(&self) -> Self {
        match self.message_type {
            GameMessageType::Ball => GameMessage {
                message_type: GameMessageType::Ball,
            },
            GameMessageType::NewPitch => GameMessage {
                message_type: GameMessageType::NewPitch,
            },
            GameMessageType::Strike => GameMessage {
                message_type: GameMessageType::Strike,
            },
            GameMessageType::Out => GameMessage {
                message_type: GameMessageType::Out,
            },
            GameMessageType::Walk => GameMessage {
                message_type: GameMessageType::Walk,
            },
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum GameMessageType {
    Strike,
    Ball,
    Out,
    Walk,
    NewPitch,
}
