use crate::api::{convert_helpers, NativeWasmType};
use liquislime_core::*;

pub trait FromGameApi: Sized {
    type Api: NativeWasmType;

    fn from_game_api(input: Self::Api) -> Self;
}

impl<T: NativeWasmType> FromGameApi for T {
    type Api = Self;

    fn from_game_api(input: Self::Api) -> Self {
        input
    }
}

impl FromGameApi for bool {
    type Api = u32;

    fn from_game_api(input: Self::Api) -> Self {
        input != 0
    }
}

impl FromGameApi for Position {
    type Api = u64;

    fn from_game_api(input: Self::Api) -> Self {
        let (x, y) = convert_helpers::unpack_u32s(input);
        Position {
            x: f32::from_bits(x),
            y: f32::from_bits(y),
        }
    }
}

impl FromGameApi for TilePosition {
    type Api = u64;

    fn from_game_api(input: Self::Api) -> Self {
        let (x, y) = convert_helpers::unpack_u32s(input);
        TilePosition {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl FromGameApi for FactionId {
    type Api = u32;

    fn from_game_api(input: Self::Api) -> Self {
        FactionId { id: input as _ }
    }
}

impl FromGameApi for InputKey {
    type Api = u32;

    fn from_game_api(input: Self::Api) -> Self {
        match input {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::Up,
            3 => Self::Down,
            4 => Self::LeftMouse,
            5 => Self::RightMouse,
            6 => Self::MouseWheelUp,
            7 => Self::MouseWheelDown,
            _ => todo!("Invalid input key: {}", input),
        }
    }
}
