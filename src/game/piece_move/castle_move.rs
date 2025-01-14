use crate::game::{board::Board, position::Position};

use super::ChessMove;

#[derive(Clone)]
pub struct CastleMove {
    pub from_pos: Position,
    pub to_pos: Position,
}

impl CastleMove {
    pub fn new(from_pos: Position, to_pos: Position) -> Self {
        Self {
            from_pos,
            to_pos
        }
    }
}

impl ChessMove for CastleMove {
    fn execute(&self, board: &mut Board) {
        let mut piece = board[self.from_pos].clone().unwrap();

        piece.set_has_moved();


        if self.to_pos.column == 2 {
            let mut rook = board.state[0][self.to_pos.row as usize].clone().unwrap();

            rook.set_has_moved();

            board.state[0][self.to_pos.row as usize] = None;
            board.state[3][self.to_pos.row as usize] = Some(rook);
        }

        if self.to_pos.column == 6 {
            let mut rook = board.state[7][self.to_pos.row as usize].clone().unwrap();

            rook.set_has_moved();

            board.state[7][self.to_pos.row as usize] = None;
            board.state[5][self.to_pos.row as usize] = Some(rook);
        }
        
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