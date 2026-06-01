use crate::TilePosition;
use crate::{FromGameApi, ToGameApi};
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

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
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

impl FromGameApi for Position {
    type Api = u64;

    fn from_game_api(input: Self::Api) -> Self {
        let (x, y) = crate::convert::unpack_u32s(input);
        Self::new(f32::from_bits(x), f32::from_bits(y))
    }
}

impl ToGameApi for Position {
    type Api = u64;

    fn to_game_api(&self) -> Self::Api {
        let x = self.x.to_bits();
        let y = self.y.to_bits();
        crate::convert::pack_u32s(x, y)
    }
}
