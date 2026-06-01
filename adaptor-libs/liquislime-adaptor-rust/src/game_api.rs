use crate::*;

pub struct GameApi;

impl GameApi {
    pub fn add_slime_amount(position: TilePosition, faction_id: FactionId, amount: f64) {
        unsafe {
            add_slime_amount(faction_id.to_game_api(), position.to_game_api(), amount);
        }
    }

    pub fn get_mouse_world_position() -> Position {
        unsafe { Position::from_game_api(get_mouse_world_position()) }
    }

    pub fn is_key_pressed(key: InputKey) -> bool {
        unsafe { bool::from_game_api(is_key_pressed(key.to_game_api())) }
    }
}

#[link(wasm_import_module = "liquislime_api")]
unsafe extern "C" {
    // pub fn level_width() -> <i32 as FromWasmAbi>::Abi;
    // pub fn level_height() -> <i32 as FromWasmAbi>::Abi;

    // pub fn get_current_unit() -> <Unit as FromWasmAbi>::Abi;
    // pub fn get_current_instance() -> <Instance as FromWasmAbi>::Abi;

    // pub fn get_own_faction() -> <Faction as FromWasmAbi>::Abi;
    // pub fn get_own_position() -> <TilePosition as FromWasmAbi>::Abi;

    // pub fn get_slime_amount(
    //     faction: <Faction as ToWasmAbi>::Abi,
    //     position: <TilePosition as ToWasmAbi>::Abi,
    // ) -> f64;
    pub fn add_slime_amount(
        faction: <FactionId as ToGameApi>::Api,
        position: <TilePosition as ToGameApi>::Api,
        amount: f64,
    );

    pub fn get_mouse_world_position() -> <Position as FromGameApi>::Api;

    pub fn is_key_pressed(key: <InputKey as ToGameApi>::Api) -> <bool as FromGameApi>::Api;

    // pub fn log(msg: <&str as ToGameApi>::Output);
}
