use std::ops::{Index, IndexMut};

use super::{piece::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, ChessPiece, PieceType, Player}, position::Position};

use crate::game::piece::rook::Rook;

#[derive(Default, Clone)]
pub struct Board {
    pub state: [[Option<Box<dyn ChessPiece>>; 8]; 8],
    pub en_passant: Option<Position>,
}

impl Board {
    pub fn new() -> Self {
        let mut board: Board = Self::default();
        
        board.en_passant = None;


        board.state[0][0] = Some(Box::new(Rook::new(Player::Black)));
        board.state[1][0] = Some(Box::new(Knight::new(Player::Black)));
        board.state[2][0] = Some(Box::new(Bishop::new(Player::Black)));
        board.state[3][0] = Some(Box::new(Queen::new(Player::Black)));
        board.state[4][0] = Some(Box::new(King::new(Player::Black)));
        board.state[5][0] = Some(Box::new(Bishop::new(Player::Black)));
        board.state[6][0] = Some(Box::new(Knight::new(Player::Black)));
        board.state[7][0] = Some(Box::new(Rook::new(Player::Black)));

        board.state[0][7] = Some(Box::new(Rook::new(Player::White)));
        board.state[1][7] = Some(Box::new(Knight::new(Player::White)));
        board.state[2][7] = Some(Box::new(Bishop::new(Player::White)));
        board.state[3][7] = Some(Box::new(Queen::new(Player::White)));
        board.state[4][7] = Some(Box::new(King::new(Player::White)));
        board.state[5][7] = Some(Box::new(Bishop::new(Player::White)));
        board.state[6][7] = Some(Box::new(Knight::new(Player::White)));
        board.state[7][7] = Some(Box::new(Rook::new(Player::White)));


        for i in 0..8 {
            board.state[i][1] = Some(Box::new(Pawn::new(Player::Black)));
            board.state[i][6] = Some(Box::new(Pawn::new(Player::White)));
        }

        board
    }

    pub fn test_board() -> Self {
        let mut board: Board = Self::default();

        board.en_passant = None;

        board.state[0][0] = Some(Box::new(King::new(Player::Black)));
        board.state[7][7] = Some(Box::new(King::new(Player::White)));
        board.state[1][0] = Some(Box::new(Knight::new(Player::White)));
        board.state[5][1] = Some(Box::new(Knight::new(Player::Black)));
        

        board
    }

    pub fn is_inside(position: Position) -> bool {
        position.column >= 0 && position.column < 8
        && position.row >= 0 && position.row < 8
    }

    pub fn is_empty(&self, position: Position) -> bool {
        self[position].is_none()
    }

    pub fn get_check(&self, player: Player) -> bool {
        let board = self.state.clone();

        for (col, table) in board.into_iter().enumerate() {
            for (row, piece) in table.into_iter().enumerate() {
                if piece.is_none() {
                    continue;
                }

                let piece = piece.unwrap();

                if piece.get_color() == player {
                    continue;
                }

                for piece_move in piece.get_moves(Position::new(col as i32, row as i32), self) {
                    if let Some(piece) = &self[piece_move.get_to_pos()] {
                        if piece.get_piece_type() == PieceType::King {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    
    pub fn find_piece(&self, player: Player, piece_type: PieceType) -> Position {
        for (x, col) in self.state.iter().enumerate() {
            for (y, piece) in col.into_iter().enumerate() {
                if let Some(chess_piece) = piece {
                    if chess_piece.get_piece_type() == piece_type
                    && chess_piece.get_color() == player {
                        return Position::new(x as i32,y as i32);
                    }
                }
            }
        }

        panic!("{:?} has no king", player);
    }

    pub fn get_snapshot(&self) -> String {
        let mut snapshot = String::new();
        for table in self.state.iter() {
            for piece in table.into_iter() {
                if let Some(new_piece) = piece {
                    let mut letter = String::from(match new_piece.get_piece_type() {
                        PieceType::Pawn => "p",
                        PieceType::Knight => "n",
                        PieceType::Bishop => "b",
                        PieceType::Rook => "r",
                        PieceType::Queen => "q",
                        PieceType::King => "k",
                    });

                    if new_piece.get_color() == Player::White {
                        letter = letter.to_uppercase();
                    }

                    snapshot += &letter;
                }
                else {
                    snapshot += "e";
                }
            }
        }

        

        snapshot += 'letter: {
            if let Some(piece) = &self[Position::new(4, 7)] {
                if piece.get_piece_type() == PieceType::King 
                && !piece.get_has_moved() {
                    if let Some(rook) = &self[Position::new(7, 7)] {
                        if rook.get_piece_type() == PieceType::Rook
                        && !rook.get_has_moved() {
                            break 'letter "K";
                        }
                    }
                    
                }
            };

            "e"
        };

        snapshot += 'letter: {
            if let Some(piece) = &self[Position::new(4, 7)] {
                if piece.get_piece_type() == PieceType::King 
                && !piece.get_has_moved() {
                    if let Some(rook) = &self[Position::new(0, 7)] {
                        if rook.get_piece_type() == PieceType::Rook
                        && !rook.get_has_moved() {
                            break 'letter "Q";
                        }
                    }
                    
                }
            };

            "e"
        };
        

        snapshot += 'letter: {
            if let Some(piece) = &self[Position::new(4, 0)] {
                if piece.get_piece_type() == PieceType::King 
                && !piece.get_has_moved() {
                    if let Some(rook) = &self[Position::new(7, 0)] {
                        if rook.get_piece_type() == PieceType::Rook
                        && !rook.get_has_moved() {
                            break 'letter "k";
                        }
                    }
                    
                }
            };

            "e"
        };

        snapshot += 'letter: {
            if let Some(piece) = &self[Position::new(4, 0)] {
                if piece.get_piece_type() == PieceType::King 
                && !piece.get_has_moved() {
                    if let Some(rook) = &self[Position::new(0, 0)] {
                        if rook.get_piece_type() == PieceType::Rook
                        && !rook.get_has_moved() {
                            break 'letter "q";
                        }
                    }
                    
                }
            };

            "e"
        };

        // let white_king_pos = self.find_piece(Player::White, PieceType::King);
        // let white_king_moves = self[white_king_pos].clone().unwrap().get_moves(white_king_pos, self);
        // let black_king_pos = self.find_piece(Player::Black, PieceType::King);
        // let black_king_moves = self[black_king_pos].clone().unwrap().get_moves(black_king_pos, self);

        // if white_king_moves.iter().any(|one_move| {
        //     one_move.get_from_pos().row - one_move.get_to_pos().row == 2
        // }) {
        //     snapshot += "Q";
        // }
        // else {
        //     snapshot += "e";
        // }

        // if white_king_moves.iter().any(|one_move| {
        //     one_move.get_to_pos().row - one_move.get_from_pos().row == 2
        // }) {
        //     snapshot += "K";
        // }
        // else {
        //     snapshot += "e";
        // }

        

        // if black_king_moves.iter().any(|one_move| {
        //     one_move.get_from_pos().row - one_move.get_to_pos().row == 2
        // }) {
        //     snapshot += "q";
        // }
        // else {
        //     snapshot += "e";
        // }

        // if black_king_moves.iter().any(|one_move| {
        //     one_move.get_to_pos().row - one_move.get_from_pos().row == 2
        // }) {
        //     snapshot += "k";
        // }
        // else {
        //     snapshot += "e";
        // }




        println!("{}", snapshot);

        snapshot
    }
}

impl Index<Position> for Board {
    type Output = Option<Box<dyn ChessPiece>>;

    fn index(&self, index: Position) -> &Self::Output {
        &self.state[index.column as usize][index.row as usize]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.state[index.column as usize][index.row as usize]
    }
}
