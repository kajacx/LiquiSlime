use liquislime_core::*;
use wasmi::{errors::LinkerError, Caller, Linker};

use crate::{
    adaptor::StoreData,
    api::{FromGameApi, ToGameApi},
};

pub fn add_imported_game_functions(linker: &mut Linker<StoreData>) -> Result<(), LinkerError> {
    linker.func_wrap(
        "liquislime_api",
        "add_slime_amount",
        |caller: Caller<StoreData>,
         faction: <FactionId as FromGameApi>::Api,
         position: <TilePosition as FromGameApi>::Api,
         amount: f64| {
            let mut game_interaction = caller.data().game_interaction.unwrap();

            unsafe {
                game_interaction.as_mut().add_slime(
                    FactionId::from_game_api(faction),
                    TilePosition::from_game_api(position),
                    SlimeAmount::from_float(amount),
                );
            }

            Ok(())
        },
    )?;

    linker.func_wrap(
        "liquislime_api",
        "is_key_pressed",
        |caller: Caller<StoreData>, key_core: <InputKey as FromGameApi>::Api| {
            let mut game_interaction = caller.data().game_interaction.unwrap();

            let result = unsafe {
                game_interaction
                    .as_mut()
                    .is_key_pressed(InputKey::from_game_api(key_core))
            };

            Ok(result.to_game_api())
        },
    )?;

    linker.func_wrap(
        "liquislime_api",
        "get_mouse_world_position",
        |caller: Caller<StoreData>| {
            let game_interaction = unsafe { caller.data().game_interaction.unwrap().as_mut() };

            let result = game_interaction.get_mouse_world_position();

            Ok(result.to_game_api())
        },
    )?;

    Ok(())
}
