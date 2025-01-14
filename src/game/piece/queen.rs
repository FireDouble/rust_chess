use crate::game::{board::Board, direction::Direction, piece_move::{normal_move::NormalMove, ChessMove}, position::Position};

use super::{ChessPiece, PieceType, Player};

#[derive(Clone)]
pub struct Queen {
    pub piece_type: PieceType,
    pub player: Player 
}

impl Queen {
    pub fn new(player: Player) -> Self {
        Self {
            piece_type: PieceType::Queen,
            player
        }
    }
    
    pub fn get_moves_in_direction(&self, from_position: Position, board: &Board, direction: Direction) -> Vec<Box<dyn ChessMove>> {
        let mut moves = Vec::new();

        let mut pos = from_position + direction.into();

        while Board::is_inside(pos) {
            if board.is_empty(pos) {
                moves.push(Box::new(NormalMove::new(from_position, pos)) as Box<dyn ChessMove>);
                pos += direction.into();
                continue;
            }
            
            let piece = board[pos].as_ref().unwrap();
            if piece.get_color() != self.get_color() {
                moves.push(Box::new(NormalMove::new(from_position, pos)) as Box<dyn ChessMove>);
            }
            
            break;
        }

        moves
    }
}

impl ChessPiece for Queen {
    fn get_color(&self) -> Player {
        self.player
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    fn get_moves(&self, from_position: Position, board: &Board) -> Vec<Box<dyn ChessMove>> {
        let mut moves = Vec::new();

        for dir in [Direction::NORTH, Direction::NORTH_WEST, Direction::WEST, Direction::SOUTH_WEST, Direction::SOUTH, Direction::SOUTH_EAST, Direction::EAST, Direction::NORTH_EAST].into_iter() {
            moves.append(&mut self.get_moves_in_direction(from_position, board, dir));
        }
        
        moves
    }

    fn set_has_moved(&mut self) {}
}