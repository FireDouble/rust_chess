use std::collections::HashMap;

use board::Board;
use game_event::GameEvent;
use ggez::{event::MouseButton, glam::Vec2, graphics::{self, Canvas, Color, Rect}, Context};
use modals::{endgame_modal::{EndType, EndgameModal}, pawn_promotion_modal::PawnPromotionModal, Modal};
use piece::{bishop::Bishop, knight::Knight, queen::Queen, rook::Rook, PieceType, Player};
use piece_counting::PieceCounting;
use piece_move::ChessMove;
use position::Position;

use crate::{app::AppEvent, assets::Assets, scenes::Scene};

pub mod board;
pub mod piece;
pub mod position;
pub mod piece_move;
pub mod direction;
pub mod modals;
pub mod game_event;
pub mod piece_counting;

pub struct Game {
    pub board: Board,
    pub modals: Option<Box<dyn Modal>>,
    pub legal_moves: Option<Vec<Box<dyn ChessMove>>>,
    pub current_player: Player,
    pub fifty_move_rule: u8,
    pub snapshots: HashMap<String, u8>,
    pub is_checked: bool,
    pub game_events: Vec<GameEvent>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            modals: None,
            legal_moves: None,
            current_player: Player::White,
            fifty_move_rule: 0,
            snapshots: HashMap::new(),
            is_checked: false,
            game_events: Vec::new(),
        }
    }

    pub fn is_checkmate_or_stalemate(&mut self) -> bool {
        let mut found_moves: bool = false;

        'outer: for (col, table) in self.board.state.iter().enumerate() {
            for (row, piece) in table.into_iter().enumerate() {
                if let Some(piece) = piece {
                    if piece.get_color() == self.current_player {
                        let mut piece_moves = piece.get_moves(Position::new(col as i32, row as i32), &self.board);
                        piece_moves.retain(|piece_move| {
                            let mut test_board = self.board.clone();
                            
                            piece_move.execute(&mut test_board);

                            !test_board.get_check(self.current_player)
                        });

                        if !piece_moves.is_empty() {
                            found_moves = true;
                            break 'outer;
                        }
                    }
                }
            }
        }

        !found_moves        
    }

    pub fn is_king_v_king(&self, pieces: PieceCounting) -> bool {
        pieces.total_pieces == 2
    }

    pub fn is_king_v_king_knight(&self, pieces: PieceCounting) -> bool {
        pieces.total_pieces == 3 
        && (pieces.white_pieces.get(&PieceType::Knight).is_some() 
        || pieces.black_pieces.get(&PieceType::Knight).is_some())
    }

    pub fn is_king_v_king_bishop(&self, pieces: PieceCounting) -> bool {
        pieces.total_pieces == 3 
        && (pieces.white_pieces.get(&PieceType::Bishop).is_some() 
        || pieces.black_pieces.get(&PieceType::Bishop).is_some())
    }
    
    pub fn is_king_bishop_v_king_bishop(&self, pieces: PieceCounting) -> bool {
        if pieces.total_pieces == 4
        && pieces.white_pieces.get(&PieceType::Bishop).is_some() 
        && pieces.black_pieces.get(&PieceType::Bishop).is_some() {
            let white_bishop_pos = self.board.find_piece(Player::White, PieceType::Bishop);
            let black_bishop_pos = self.board.find_piece(Player::Black, PieceType::Bishop);

            if (white_bishop_pos.column + white_bishop_pos.row + black_bishop_pos.column + black_bishop_pos.row) % 2 != 0 {
                return true;
            }
        }

        false
    }

    pub fn is_insufficient_material(&self) -> bool {
        let pieces = self.get_total_pieces();

        self.is_king_v_king(pieces.clone()) || self.is_king_v_king_knight(pieces.clone())
        || self.is_king_v_king_bishop(pieces.clone()) || self.is_king_bishop_v_king_bishop(pieces.clone())
    }

    pub fn get_total_pieces(&self) -> PieceCounting {
        let mut total_pieces = 0;
        let mut black_pieces = HashMap::new();
        let mut white_pieces = HashMap::new();

        for col in self.board.state.iter() {
            for piece in col.into_iter() {
                if let Some(piece) = piece {
                    if piece.get_color() == Player::Black {
                        black_pieces.insert(piece.get_piece_type(), black_pieces.get(&piece.get_piece_type()).unwrap_or(&0) + 1);
                    }
                    else {
                        white_pieces.insert(piece.get_piece_type(), white_pieces.get(&piece.get_piece_type()).unwrap_or(&0) + 1);
                    }

                    total_pieces += 1;
                }
            }
        }

        PieceCounting {
            total_pieces,
            black_pieces,
            white_pieces
        }
    }


}

impl Scene for Game {
    fn draw(&self, assets: &Assets, canvas: &mut Canvas, ctx: &mut Context) {
        canvas.draw(assets.graphics.get("board").unwrap(), [0., 0.]);

        if let Some(legal_moves) = &self.legal_moves {
            for piece_move in legal_moves {


                if let Some(_) = self.board[piece_move.get_to_pos()] {
                    canvas.draw(
                        assets.graphics.get("capturehighlight").unwrap(),
                        [(32*piece_move.get_to_pos().column+16) as f32, (32*piece_move.get_to_pos().row+16) as f32]
                    )
                } else {
                    canvas.draw(
                        assets.graphics.get("movehighlight").unwrap(),
                        [(32*piece_move.get_to_pos().column+16) as f32, (32*piece_move.get_to_pos().row+16) as f32]
                    )
                }
            }
        }





        if self.is_checked {
            let king_pos = self.board.find_piece(self.current_player, PieceType::King);

            canvas.draw(assets.graphics.get("checkhighlight").unwrap(), [(32*king_pos.column+16) as f32, (32*king_pos.row+16) as f32]);
        }

        for (col_index, col) in self.board.state.iter().enumerate() {
            for (row_index, maybe_piece) in col.into_iter().enumerate() {
                if let Some(piece) = maybe_piece {
                    let piece_asset_type = match piece.get_piece_type() {
                        PieceType::Pawn => "pawn",
                        PieceType::Knight => "knight",
                        PieceType::Bishop => "bishop",
                        PieceType::Rook => "rook",
                        PieceType::Queen => "queen",
                        PieceType::King => "king",
                    };

                    let piece_asset_player = match piece.get_color() {
                        Player::Black => "black",
                        Player::White => "white",
                    };

                    let piece_asset_id = String::from(piece_asset_player) + piece_asset_type;

                    canvas.draw(assets.graphics.get(&piece_asset_id).unwrap(), [(32*col_index+16) as f32, (32*row_index+16) as f32]);
                }
            }
        }

        

        if let Some(modal) = self.modals.as_ref() {
            canvas.draw(&graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., 288., 288.),
                Color::new(1., 1., 1., 0.05)
            ).unwrap(), [0., 0.]);

            modal.draw(canvas, ctx);
        }
    }

    fn update(&mut self, press_data: Option<(Vec2, MouseButton)>, ctx: &mut Context, app_events: &mut Vec<AppEvent>) {
        if let Some(game_modal) = &self.modals {
            if let Some(message) = game_modal.check_for_message(press_data.map(|(pos, _)| pos)) {
                self.game_events.push(message);
                self.modals = None;
            }
        }
        else {
            let events: Vec<GameEvent> = self.game_events.drain(..).collect();
            for event in events {
                match event {
                    GameEvent::Replay => app_events.push(AppEvent::OpenNewGame),
                    GameEvent::Exit => app_events.push(AppEvent::OpenMainMenu),
                    GameEvent::ChoosePiece(piece_type, position) => {
                        self.modals = None;
                        
                        let old_piece = self.board[position].clone().unwrap();
    
                        self.board[position] = match piece_type {
                            PieceType::Knight => Some(Box::new(Knight::new(old_piece.get_color()))),
                            PieceType::Bishop => Some(Box::new(Bishop::new(old_piece.get_color()))),
                            PieceType::Rook => Some(Box::new(Rook::new(old_piece.get_color()))),
                            PieceType::Queen => Some(Box::new(Queen::new(old_piece.get_color()))),
                            _ => None,
                        };
                        
                        self.game_events.push(GameEvent::CheckForGameEnd);
                    },
                    GameEvent::CheckForGameEnd => {
                        println!("{:?}", self.get_total_pieces());
                        println!("{:?}", self.fifty_move_rule);

                        let snapshot = self.board.get_snapshot();

                        if let Some(count) = self.snapshots.get_mut(&snapshot) {
                            *count += 1;
                        }
                        else {
                            self.snapshots.insert(snapshot.clone(), 1);
                        }

                        if *self.snapshots.get(&snapshot).unwrap() == 3 {
                            self.modals = Some(Box::new(EndgameModal::new(EndType::Repetition, ctx)));
                        }

                        self.is_checked = self.board.get_check(self.current_player);

                        if self.is_checkmate_or_stalemate() {
                            if self.is_checked {
                                self.modals = Some(Box::new(EndgameModal::new(EndType::Checkmate(self.current_player.get_opponent()), ctx)));
                            }
                            else {
                                self.modals = Some(Box::new(EndgameModal::new(EndType::Stalemate, ctx)));
                            }
                        }

                        if self.is_insufficient_material() {
                            self.modals = Some(Box::new(EndgameModal::new(EndType::InsufficientMaterial, ctx)));
                        }

                        if self.fifty_move_rule >= 100 {
                            self.modals = Some(Box::new(EndgameModal::new(EndType::FiftyMoveRule, ctx)));
                        }
                    },
                }
            }
            

            if let Some((press_position, button)) = press_data {
                if button == MouseButton::Right {
                    self.legal_moves = None;
                }
    
                if press_position[0] > 16. && press_position[0] < 272. 
                && press_position[1] > 16. && press_position[1] < 272. {
                    let column = ((press_position[0] - 16.) / 32.) as usize;
                    let row = ((press_position[1] - 16.) / 32.) as usize;
    
                    let pos = Position::new(column as i32, row as i32);
                    let piece = self.board[pos].clone();
    
                    if let Some(legal_moves) = self.legal_moves.clone() {
                        for piece_move in legal_moves {
                            if piece_move.get_to_pos() == pos {
                                self.fifty_move_rule += 1;
                                if self.board[piece_move.get_to_pos()].is_some()
                                || self.board[piece_move.get_from_pos()].clone().unwrap().get_piece_type() == PieceType::Pawn {
                                    self.fifty_move_rule = 0;
                                }
                                
                                piece_move.execute(&mut self.board);

                                if piece_move.is_pawn_promotion_move() {
                                    self.modals = Some(Box::new(PawnPromotionModal::new(ctx, piece_move.get_to_pos())) as Box<dyn Modal>)
                                }
                                
                                self.legal_moves = None;
                                self.current_player = self.current_player.get_opponent();

                                self.game_events.push(GameEvent::CheckForGameEnd);
                            }
                        }
                    } else {
                        if let Some(piece) = piece {
                            if self.current_player == piece.get_color() {
                                let mut piece_moves = piece.get_moves(pos, &self.board);
    
                                piece_moves.retain(|piece_move| {
                                    let mut test_board = self.board.clone();
                                    
                                    piece_move.execute(&mut test_board);
    
                                    !test_board.get_check(self.current_player)
                                });
    
                                self.legal_moves = Some(piece_moves);
                            }
                        }
                    }
                } 
            }
        }
    }
}