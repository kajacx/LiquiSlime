use crate::convert::NativeWasmType;

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
        if *self { 1 } else { 0 }
    }
}
