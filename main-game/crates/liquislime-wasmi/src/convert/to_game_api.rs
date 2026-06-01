use crate::convert::{convert_helpers, NativeWasmType};
use liquislime_core::*;

pub trait ToGameApi {
    type Api: NativeWasmType;

    fn to_game_api(&self) -> Self::Api;
}

impl<T: NativeWasmType + Copy> ToGameApi for T {
    type Api = Self;

    fn to_game_api(&self) -> Self::Api {
        *self
    }
}

impl ToGameApi for bool {
    type Api = u32;

    fn to_game_api(&self) -> Self::Api {
        if *self {
            1
        } else {
            0
        }
    }
}

impl ToGameApi for Position {
    type Api = u64;

    fn to_game_api(&self) -> Self::Api {
        convert_helpers::pack_u32s(self.x.to_bits(), self.y.to_bits())
    }
}

impl ToGameApi for TilePosition {
    type Api = u64;

    fn to_game_api(&self) -> Self::Api {
        convert_helpers::pack_u32s(self.x as u32, self.y as u32)
    }
}

impl ToGameApi for InputKey {
    type Api = u32;

    fn to_game_api(&self) -> Self::Api {
        match self {
            Self::Left => 0,
            Self::Right => 1,
            Self::Up => 2,
            Self::Down => 3,
            Self::LeftMouse => 4,
            Self::RightMouse => 5,
            Self::MouseWheelUp => 6,
            Self::MouseWheelDown => 7,
        }
    }
}
