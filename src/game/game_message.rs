pub struct GameMessage {
    pub message_type: GameMessageType,
}

#[derive(Debug)]
pub enum GameMessageType {
    Strike,
    Ball,
    Out,
    Walk,
    NewPitch,
}
