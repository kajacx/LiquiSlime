use crate::{FromGameApi, ToGameApi};

pub enum InputKey {
    Left,
    Right,
    Up,
    Down,
    LeftMouse,
    RightMouse,
    MouseWheelUp,
    MouseWheelDown,
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
