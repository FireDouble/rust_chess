use super::piece::Player;
use derive_more::{Add, AddAssign};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Add, AddAssign, Debug)]
pub struct Position {
    pub column: i32,
    pub row: i32,
}

impl Position {
    pub fn new(column: i32, row: i32) -> Self {
        Self {
            column,
            row
        }
    }

    pub fn square_color(&self) -> Player {
        if (self.column + self.row) % 2 == 0 {
            return Player::White;
        } 
        
        Player::Black
    }
}