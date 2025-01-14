use super::{board::Board, piece_move::ChessMove, position::Position};

pub mod pawn;
pub mod knight;
pub mod bishop;
pub mod rook;
pub mod queen;
pub mod king;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}
// #[default] White
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    White,
    Black
}

impl Player {
    pub fn get_opponent(&self) -> Self {
        if *self == Self::White {
            return Self::Black;
        }

        Self::White
    }
}

pub trait ChessPiece: dyn_clone::DynClone {
    fn get_color(&self) -> Player;

    fn get_piece_type(&self) -> PieceType;

    fn get_moves(&self, from_position: Position, board: &Board) -> Vec<Box<dyn ChessMove>>;

    fn get_has_moved(&self) -> bool {
        false
    }

    fn set_has_moved(&mut self); 
}

dyn_clone::clone_trait_object!(ChessPiece);