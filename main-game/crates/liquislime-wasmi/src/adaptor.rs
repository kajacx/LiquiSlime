use std::ptr::NonNull;

use liquislime_core::*;
use wasmi::{Caller, Instance, Linker, Module, Store, Val, F64};

use crate::convert::{FromGameApi, ToGameApi};

pub struct WasmiAdaptor {
    store: Store<StoreData>,
    instance: Instance,
}

#[derive(Default)]
struct StoreData {
    game_interaction: Option<NonNull<GameInteraction<'static>>>,
}

impl WasmiAdaptor {
    pub fn new(bytes: &[u8]) -> Self {
        let mut store = Store::default();

        let module =
            Module::new(store.engine(), bytes).expect("TODO: Failed to create module from bytes");

        let mut linker = Linker::<StoreData>::new(&store.engine());

        linker
            .func_wrap(
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
            )
            .expect("TODO: Failed to define 'get_game_interaction' function");

        linker
            .func_wrap(
                "liquislime_api",
                "is_key_pressed",
                |caller: Caller<StoreData>, key_core: <InputKey as FromGameApi>::Api| {
                    let mut game_interaction = caller.data().game_interaction.unwrap();

                    let result = unsafe {
                        game_interaction
                            .as_mut()
                            .is_key_pressed(InputKey::from_game_api(key_core))
                    };

                    println!(
                        "is_key_pressed called with key_core: {}, result: {}",
                        key_core, result
                    );

                    Ok(result.to_game_api())
                },
            )
            .expect("TODO: Failed to define 'is_key_pressed' function");

        linker
            .func_wrap(
                "liquislime_api",
                "get_mouse_world_position",
                |caller: Caller<StoreData>| {
                    let game_interaction =
                        unsafe { caller.data().game_interaction.unwrap().as_mut() };

                    let result = game_interaction.get_mouse_world_position();

                    Ok(result.to_game_api())
                },
            )
            .expect("TODO: Failed to define 'get_mouse_world_position' function");

        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("TODO: Failed to create instance from module");

        Self { store, instance }
    }
}

impl BehaviourAdaptor for WasmiAdaptor {
    fn update(&mut self, game_interaction: &mut GameInteraction, time_passed: TimeInterval) {
        println!("Updating Wasmi adaptor");

        unsafe {
            let game_interaction: &'static mut GameInteraction<'static> =
                std::mem::transmute(game_interaction);

            self.store.data_mut().game_interaction = Some(NonNull::from_mut(game_interaction));
        }

        self.instance
            .get_export(&self.store, "update")
            .expect("TODO: Failed to find 'update' export")
            .into_func()
            .expect("TODO: func")
            .call(
                &mut self.store,
                &[Val::F64(F64::from_float(time_passed.to_seconds()))],
                &mut [],
            )
            .expect("TODO: Failed to invoke 'update' export");

        self.store.data_mut().game_interaction = None;
    }
}
