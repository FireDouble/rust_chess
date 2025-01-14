use crate::game::{board::Board, position::Position};

use super::ChessMove;

#[derive(Clone)]
pub struct NormalMove {
    pub from_pos: Position,
    pub to_pos: Position,
}

impl NormalMove {
    pub fn new(from_pos: Position, to_pos: Position) -> Self {
        Self {
            from_pos,
            to_pos
        }
    }
}

impl ChessMove for NormalMove {
    fn execute(&self, board: &mut Board) {
        let mut piece = board[self.from_pos].clone().unwrap();

        piece.set_has_moved();
        
        board[self.from_pos] = None;
        board[self.to_pos] = Some(piece);

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