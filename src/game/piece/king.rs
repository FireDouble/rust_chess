use crate::game::{board::Board, direction::Direction, piece_move::{castle_move::CastleMove, normal_move::NormalMove, ChessMove}, position::Position};

use super::{ChessPiece, PieceType, Player};

#[derive(Clone)]
pub struct King {
    pub piece_type: PieceType,
    pub player: Player,
    pub has_moved: bool
}

impl King {
    pub fn new(player: Player) -> Self {
        Self {
            piece_type: PieceType::King,
            player,
            has_moved: false
        }
    }

    pub fn get_moves_in_direction(&self, from_position: Position, board: &Board, direction: Direction)-> Vec<Box<dyn ChessMove>> {
        let mut moves = Vec::new();

        let mut pos = from_position + direction.into();

        if Board::is_inside(pos) {
            if board.is_empty(pos) {
                moves.push(Box::new(NormalMove::new(from_position, pos)) as Box<dyn ChessMove>);
                pos += direction.into();
            }
            
            else {
                let piece = board[pos].as_ref().unwrap();
                if piece.get_color() != self.get_color() {
                    moves.push(Box::new(NormalMove::new(from_position, pos)) as Box<dyn ChessMove>);
                }
            }
        }

        moves
    }
}

impl ChessPiece for King {
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

        if !self.has_moved {
            let row = if self.player == Player::White {7} else {0};
            
            let piece = board.state[0][row].clone();

            if let Some(piece) = piece {
                if piece.get_piece_type() == PieceType::Rook 
                && piece.get_color() == self.player {
                    let mut empty = true;
                    
                    if !piece.get_has_moved() {
                        for i in 0..3 {
                            if !board.is_empty(from_position + (Direction::WEST * (i + 1)).into()) {
                                empty = false;
                                break;
                            }

                            let mut test_board = board.clone();
                            let test_move = NormalMove::new(
                                from_position,
                                from_position + (Direction::WEST).into()
                            );

                            test_move.execute(&mut test_board);

                            if test_board.get_check(self.player) {
                                empty = false;
                                break;
                            }
                        }

                        if empty {
                            moves.push(Box::new(CastleMove::new(
                                from_position,
                                from_position + (Direction::WEST * 2).into()
                            )));
                        }
                    }
                }
            }

            let piece = board.state[7][row].clone();

            if let Some(piece) = piece {
                if piece.get_piece_type() == PieceType::Rook 
                && piece.get_color() == self.player {
                    if !piece.get_has_moved() {
                        let mut empty = true;
                        for i in 0..2 {
                            if !board.is_empty(from_position + (Direction::EAST * (i + 1)).into()) {
                                empty = false;
                                break;
                            }

                            let mut test_board = board.clone();
                            let test_move = NormalMove::new(
                                from_position,
                                from_position + (Direction::EAST).into()
                            );

                            test_move.execute(&mut test_board);

                            if test_board.get_check(self.player) {
                                empty = false;
                                break;
                            }
                        }

                        if empty {
                            moves.push(Box::new(CastleMove::new(
                                from_position,
                                from_position + (Direction::EAST * 2).into()
                            )));
                        }
                    }
                }
            }
        }


        // let color_index = if board[from_position].as_ref().unwrap().get_color() == Player::White {0} else {1};

        // if board.castle[color_index][0] {
        //     let mut empty = true;
        //     for i in 0..3 {
        //         if !board.is_empty(from_position + (Direction::WEST * (i + 1)).into()) {
        //             empty = false;
        //         }
        //     }

        //     if empty {
        //         moves.push(Box::new(CastleMove::new(
        //             from_position,
        //             from_position + (Direction::WEST * 2).into()
        //         )));
        //     }
        // }
        // if board.castle[color_index][1] {
        //     let mut empty = true;
        //     for i in 0..2 {
        //         if !board.is_empty(from_position + (Direction::EAST * (i + 1)).into()) {
        //             empty = false;
        //         }
        //     }

        //     if empty {
        //         moves.push(Box::new(CastleMove::new(
        //             from_position,
        //             from_position + (Direction::EAST * 2).into()
        //         )));
        //     }
        // }
        
        moves
    }

    fn get_has_moved(&self) -> bool {
        self.has_moved
    }

    fn set_has_moved(&mut self) {
        self.has_moved = true
    }
}