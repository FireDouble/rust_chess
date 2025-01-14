use crate::game::{board::Board, direction::Direction, piece::Player, position::Position};

use super::ChessMove;

#[derive(Clone)]
pub struct EnPassantMove {
    pub from_pos: Position,
    pub to_pos: Position,
}

impl EnPassantMove {
    pub fn new(from_pos: Position, to_pos: Position) -> Self {
        Self {
            from_pos,
            to_pos
        }
    }
}

impl ChessMove for EnPassantMove {
    fn execute(&self, board: &mut Board) {
        let mut piece = board[self.from_pos].clone().unwrap();

        piece.set_has_moved();

        let direction = if piece.as_ref().get_color() == Player::White {Direction::SOUTH} else {Direction::NORTH};
        
        board[self.from_pos] = None;
        board[self.to_pos] = Some(piece);
        board[self.to_pos + direction.into()] = None;

        board.en_passant = None;
    }

    fn get_to_pos(&self) -> Position {
        self.to_pos
    }
    
    fn get_from_pos(&self) -> Position {
        self.from_pos
    }
    
    fn is_pawn_promotion_move(&self) -> bool {
        return false
    }
}