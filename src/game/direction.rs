use derive_more::{Add, AddAssign, Mul};

use super::position::Position;

#[derive(Clone, Copy, Add, AddAssign, Mul)]
pub struct Direction {
    pub column_delta: i32,
    pub row_delta: i32,
}

impl Direction {
    pub const fn new(column_delta: i32, row_delta: i32) -> Self {
        Self {
            column_delta,
            row_delta
        }
    }
}

impl Into<Position> for Direction {
    fn into(self) -> Position {
        Position::new(self.column_delta, self.row_delta)
    }
}

impl Direction {
    pub const NORTH: Self = Self::new(0, -1);
    pub const SOUTH: Self = Self::new(0, 1);
    pub const EAST: Self = Self::new(1, 0);
    pub const WEST: Self = Self::new(-1, 0);

    pub const NORTH_EAST: Self = Self::new(
        Self::NORTH.row_delta + Self::EAST.row_delta, 
        Self::NORTH.column_delta + Self::EAST.column_delta, 
    );
    pub const SOUTH_EAST: Self = Self::new(
        Self::SOUTH.row_delta + Self::EAST.row_delta, 
        Self::SOUTH.column_delta + Self::EAST.column_delta, 
    );
    pub const NORTH_WEST: Self = Self::new(
        Self::NORTH.row_delta + Self::WEST.row_delta, 
        Self::NORTH.column_delta + Self::WEST.column_delta, 
    );
    pub const SOUTH_WEST: Self = Self::new(
        Self::SOUTH.row_delta + Self::WEST.row_delta, 
        Self::SOUTH.column_delta + Self::WEST.column_delta, 
    );
}