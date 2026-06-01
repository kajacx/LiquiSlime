use crate::convert::NativeWasmType;

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
