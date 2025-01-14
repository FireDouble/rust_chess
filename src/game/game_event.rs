use super::{piece::PieceType, position::Position};

pub enum GameEvent {
    Replay,
    Exit,
    ChoosePiece(PieceType, Position),
    CheckForGameEnd
}