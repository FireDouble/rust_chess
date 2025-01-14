use std::collections::HashMap;

use super::piece::PieceType;

#[derive(Debug, Clone)]
pub struct PieceCounting {
    pub total_pieces: i32,
    pub black_pieces: HashMap<PieceType, i8>,
    pub white_pieces: HashMap<PieceType, i8>
}