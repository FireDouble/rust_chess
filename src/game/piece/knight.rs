use crate::game::{board::Board, direction::Direction, piece_move::{normal_move::NormalMove, ChessMove}, position::Position};

use super::{ChessPiece, PieceType, Player};

#[derive(Clone)]
pub struct Knight {
    pub piece_type: PieceType,
    pub player: Player 
}

impl Knight {
    pub fn new(player: Player) -> Self {
        Self {
            piece_type: PieceType::Knight,
            player
        }
    }
}

impl ChessPiece for Knight {
    fn get_color(&self) -> Player {
        self.player
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    fn get_moves(&self, from_position: Position, board: &Board) -> Vec<Box<dyn ChessMove>> {
        let mut moves = Vec::new();

        for vertical_dir in [Direction::NORTH, Direction::SOUTH] {
            for horizontal_dir in [Direction::EAST, Direction::WEST] {
                moves.push(Box::new(NormalMove::new(
                    from_position, 
                    from_position + (vertical_dir * 2).into() + horizontal_dir.into()
                )) as Box<dyn ChessMove>);
                moves.push(Box::new(NormalMove::new(
                    from_position, 
                    from_position + (horizontal_dir * 2).into() + vertical_dir.into()
                )) as Box<dyn ChessMove>);
            }
        }

        moves.into_iter()
            .filter(|piece_move|
                Board::is_inside(piece_move.get_to_pos()) 
                    && (board.is_empty(piece_move.get_to_pos()) || board[piece_move.get_to_pos()].as_ref().unwrap().get_color() != self.get_color())
            )
            .collect()
    }

    fn set_has_moved(&mut self) {}
}