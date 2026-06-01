use crate::{FromGameApi, ToGameApi};
use derive_more::{Add, Sub};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Add, Sub)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

impl TilePosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl FromGameApi for TilePosition {
    type Api = u64;

    fn from_game_api(input: Self::Api) -> Self {
        let (x, y) = crate::convert::unpack_u32s(input);
        Self::new(x as i32, y as i32)
    }
}

impl ToGameApi for TilePosition {
    type Api = u64;

    fn to_game_api(&self) -> Self::Api {
        let x = self.x as u32;
        let y = self.y as u32;
        crate::convert::pack_u32s(x, y)
    }
}
