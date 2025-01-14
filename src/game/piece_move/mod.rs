use super::{board::Board, position::Position};

pub mod normal_move;
pub mod double_pawn_move;
pub mod en_passant_move;
pub mod castle_move;
pub mod pawn_promotion_move;

pub trait ChessMove: dyn_clone::DynClone {
    fn execute(&self, board: &mut Board);
    fn get_to_pos(&self) -> Position;
    fn get_from_pos(&self) -> Position;
    fn is_pawn_promotion_move(&self) -> bool;
}

dyn_clone::clone_trait_object!(ChessMove);