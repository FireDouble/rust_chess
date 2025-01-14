use crate::game::{board::Board, direction::Direction, piece::Player, position::Position};

use super::ChessMove;

#[derive(Clone)]
pub struct DoublePawnMove {
    pub from_pos: Position,
    pub to_pos: Position,
}

impl DoublePawnMove {
    pub fn new(from_pos: Position, to_pos: Position) -> Self {
        Self {
            from_pos,
            to_pos
        }
    }
}

impl ChessMove for DoublePawnMove {
    fn execute(&self, board: &mut Board) {
        let mut piece = board[self.from_pos].clone().unwrap();
        
        piece.set_has_moved();

        let direction = if piece.clone().as_ref().get_color() == Player::White {Direction::NORTH} else {Direction::SOUTH};
        let pos = self.from_pos + direction.into();

        board.en_passant = Some(pos);
        
        board[self.from_pos] = None;
        board[self.to_pos] = Some(piece);
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