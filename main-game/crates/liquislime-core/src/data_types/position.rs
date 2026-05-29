use crate::TilePosition;
use derive_more::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use std::ops::{Div, Mul};

#[derive(Debug, Clone, Default, Copy, PartialEq, Add, AddAssign, Sub, SubAssign, Mul, Div)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn to_tile_position(self) -> TilePosition {
        TilePosition::new(self.x.floor() as i32, self.y.floor() as i32)
    }
}

impl Mul for Position {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for Position {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
