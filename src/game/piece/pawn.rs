use crate::game::{board::Board, direction::Direction, piece_move::{double_pawn_move::DoublePawnMove, en_passant_move::EnPassantMove, normal_move::NormalMove, pawn_promotion_move::PawnPromotionMove, ChessMove}, position::Position};

use super::{ChessPiece, PieceType, Player};

#[derive(Clone)]
pub struct Pawn {
    pub piece_type: PieceType,
    pub player: Player,
    pub has_moved: bool
}

impl Pawn {
    pub fn new(player: Player) -> Self {
        Self {
            piece_type: PieceType::Pawn,
            player,
            has_moved: false
        }
    }
}

impl ChessPiece for Pawn {
    fn get_color(&self) -> Player {
        self.player
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    fn get_moves(&self, from_position: Position, board: &Board) -> Vec<Box<dyn ChessMove>> {
        let mut moves = Vec::new();
        let mut captures = Vec::new();

        let direction = if self.get_color() == Player::White { Direction::NORTH } else { Direction::SOUTH };

        moves.push(Box::new(NormalMove::new(
            from_position, 
            from_position + direction.into()
        )) as Box<dyn ChessMove>);
        if !self.has_moved
        && board.is_empty(from_position + direction.into()) {
            moves.push(Box::new(DoublePawnMove::new(
                from_position, 
                from_position + (direction * 2).into()
            )) as Box<dyn ChessMove>);
        }

        captures.push(Box::new(NormalMove::new(
            from_position,
            from_position + (direction + Direction::WEST).into()
        )) as Box<dyn ChessMove>);
        captures.push(Box::new(NormalMove::new(
            from_position,
            from_position + (direction + Direction::EAST).into()
        )) as Box<dyn ChessMove>);

        if let Some(en_passant) = board.en_passant {
            if en_passant == from_position + (direction + Direction::WEST).into()
            || en_passant == from_position + (direction + Direction::EAST).into() {
                moves.push(Box::new(EnPassantMove::new(
                    from_position,
                    en_passant
                )) as Box<dyn ChessMove>);
            }
        }


        moves = moves.into_iter()
            .filter(|piece_move|
                Board::is_inside(piece_move.get_to_pos()) 
                    && board.is_empty(piece_move.get_to_pos())
            )
            .collect();

        captures = captures.into_iter()
            .filter(|piece_move|
                Board::is_inside(piece_move.get_to_pos()) 
                    && !board.is_empty(piece_move.get_to_pos())
                    && board[piece_move.get_to_pos()].as_ref().unwrap().get_color() != self.get_color()
            )
            .collect();

        moves.append(&mut captures);

        moves = moves.into_iter().map(|piece_move| {
            if piece_move.get_to_pos().row == 7
            || piece_move.get_to_pos().row == 0 {
                return Box::new(PawnPromotionMove::new(
                    from_position,
                    piece_move.get_to_pos()
                )) as Box<dyn ChessMove>
            }
            return piece_move
        }).collect();
            

        moves
    }

    fn get_has_moved(&self) -> bool {
        self.has_moved
    }

    fn set_has_moved(&mut self) {
        self.has_moved = true
    }
}