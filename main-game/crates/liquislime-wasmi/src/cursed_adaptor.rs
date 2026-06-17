use std::{any::Any, ptr::NonNull};

use liquislime_core::*;
use wasmi::{
    Caller, Engine, Instance, Linker, Module, Store, TypedResumableCall,
    TypedResumableCallHostTrap, Val,
};
use wasmi_wasi::{WasiCtx, WasiDir};

pub struct WasmiCursedAdaptor {
    store: Store<StoreData>,
    #[allow(unused)]
    instance: Instance,
    trap: Option<TypedResumableCallHostTrap<()>>,
}

pub(crate) struct StoreData {
    pub(crate) game_interaction: Option<NonNull<GameInteraction<'static>>>,
    pub(crate) ctx: WasiCtx,
}

struct EmptyDir;

impl WasiDir for EmptyDir {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl WasmiCursedAdaptor {
    pub fn new(bytes: &[u8]) -> Self {
        // 1. CREATE A PURE IN-MEMORY VIRTUAL FILESYSTEM
        // This acts like a ramdisk. It has no access to your host machine's files.
        // let memory_fs = rsfs::mem::unix::FS::default();

        // // 2. CONVERT IT TO A WASI DIR HANDLE
        // let virtual_dir = Dir::from_cap_std_dir(cap_std::fs::Dir::from_root(memory_fs));

        let ctx = wasmi_wasi::WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            // .preopened_dir(dir, guest_path)
            // .preopened_dir(virtual_dir, ".")
            // .preopened_dir(
            //     Dir::open_ambient_dir(".", ambient_authority()).unwrap(),
            //     ".",
            // )
            // .unwrap()
            .build();

        ctx.push_preopened_dir(Box::new(EmptyDir), ".").unwrap();

        let store_data = StoreData {
            game_interaction: None,
            ctx,
        };

        let mut store = Store::new(&Engine::default(), store_data);

        let module =
            Module::new(store.engine(), bytes).expect("TODO: Failed to create module from bytes");

        let mut linker = Linker::<StoreData>::new(&store.engine());

        wasmi_wasi::add_to_linker(&mut linker, |store: &mut StoreData| &mut store.ctx)
            .expect("TODO: Failed to add WASI to linker");

        linker.allow_shadowing(true);

        linker
            .func_wrap(
                "wasi_snapshot_preview1",
                "path_filestat_get",
                |caller: Caller<StoreData>,
                 _fd: i32,
                 _flags: i32,
                 path_ptr: i32,
                 path_len: i32,
                 _buf_ptr: i32| {
                    println!("[Host]: Custom 'path_filestat_get' called!");

                    // 1. Extract the path string from WASM memory
                    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                    let data = memory.data(&caller);
                    let start = path_ptr as usize;
                    let end = start + (path_len as usize);

                    if let Some(bytes) = data.get(start..end) {
                        if let Ok(path_str) = std::str::from_utf8(bytes) {
                            println!(
                                "[Rust Host] Intercepted path_open request for: {}",
                                path_str
                            );

                            if path_str.contains("ls_flow-sleep_until_next_update") {
                                return Err(wasmi::Error::new("sleep_until_next_update called"));
                            }

                            if path_str.starts_with("ls_api-get_mouse_position") {
                                let game_interaction =
                                    unsafe { caller.data().game_interaction.unwrap().as_mut() };

                                let position = game_interaction.get_mouse_world_position().x as i32;
                                println!("Passing back position: {position}");

                                return Ok(position);
                            }
                        }
                    }

                    // Standard WASI ENOENT (File Not Found) fallback
                    Ok(44)
                },
            )
            .unwrap();

        // linker
        //     .func_wrap(
        //         "wasi_snapshot_preview1",
        //         "fd_write",
        //         |mut caller: Caller<'_, StoreData>,
        //          fd: i32,
        //          iovs_ptr: i32,
        //          iovs_len: i32,
        //          nwritten_ptr: i32|
        //          -> i32 {
        //             let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

        //             println!("CALLING PRINTLN, WILL AT LEAST THIS WORK PLEASE?????");

        //             // We only care about stdout (1) and stderr (2)
        //             if fd == 1 || fd == 2 {
        //                 let mut total_written = 0;
        //                 let mut combined_text = String::new();

        //                 // WASI passes an array of 'ciovec' structures:
        //                 // Each ciovec is 8 bytes total (4 bytes for buffer pointer, 4 bytes for length)
        //                 for i in 0..iovs_len {
        //                     let ciovec_base = (iovs_ptr + (i * 8)) as usize;

        //                     // Read the pointer to the text and its length from memory
        //                     let mut ptr_bytes = [0u8; 4];
        //                     let mut len_bytes = [0u8; 4];

        //                     memory.read(&caller, ciovec_base, &mut ptr_bytes).unwrap();
        //                     memory
        //                         .read(&caller, ciovec_base + 4, &mut len_bytes)
        //                         .unwrap();

        //                     let buf_ptr = u32::from_le_bytes(ptr_bytes) as usize;
        //                     let buf_len = u32::from_le_bytes(len_bytes) as usize;

        //                     // Read the actual text string bytes
        //                     let mut text_bytes = vec![0u8; buf_len];
        //                     memory.read(&caller, buf_ptr, &mut text_bytes).unwrap();

        //                     if let Ok(text) = std::str::from_utf8(&text_bytes) {
        //                         combined_text.push_str(text);
        //                     }
        //                     total_written += buf_len as i32;
        //                 }

        //                 // FORCE PRINT directly to your actual native terminal!
        //                 let stream_name = if fd == 1 { "STDOUT" } else { "STDERR" };
        //                 print!("[Mono {}] {}", stream_name, combined_text);

        //                 // Write back how many bytes we processed to avoid throwing off Mono's counter
        //                 let mut written_bytes = [0u8; 4];
        //                 written_bytes.copy_from_slice(&(total_written as u32).to_le_bytes());
        //                 memory
        //                     .write(&mut caller, nwritten_ptr as usize, &written_bytes)
        //                     .unwrap();

        //                 return 0; // WASI Success
        //             }

        //             // Fallback or dummy handle if it's hitting another stream descriptor
        //             0
        //         },
        //     )
        //     .unwrap();

        linker.allow_shadowing(false);
        // linker
        //     .func_wrap(
        //         "wasi_snapshot_preview1",
        //         "adapter_close_badfd",
        //         |_caller: Caller<_>, _val: i32| {
        //             println!("adapter_close_badfd called: {_val}");
        //             0
        //         },
        //     )
        //     .unwrap();

        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("TODO: Failed to create instance from module");

        let result = instance
            .get_export(&store, "_start")
            .expect("Get _start export")
            .into_func()
            .expect("TODO: func")
            .typed::<(), ()>(&store)
            .expect("TODO: typed")
            .call_resumable(&mut store, ());

        if let Ok(TypedResumableCall::HostTrap(trap)) = result {
            Self {
                store,
                instance,
                trap: Some(trap),
            }
        } else {
            panic!("_start function call did not result in a trap")
        }
    }
}

impl BehaviourAdaptor for WasmiCursedAdaptor {
    fn update(&mut self, game_interaction: &mut GameInteraction, time_elapsed: TimeInterval) {
        println!("[HOST] [WasmiCursedAdaptor] Calling update");

        unsafe {
            let game_interaction = NonNull::from_mut(game_interaction.with_static_lifetime());
            self.store.data_mut().game_interaction = Some(game_interaction);
        }

        let millis = time_elapsed.to_milliseconds() as i32;
        let trap_taken = self.trap.take().unwrap();
        let result = trap_taken.resume(&mut self.store, &[Val::from(millis)]);

        if let Ok(TypedResumableCall::HostTrap(new_trap)) = result {
            self.trap = Some(new_trap)
        } else {
            panic!("_start function call did not result in a trap")
        }

        self.store.data_mut().game_interaction = None;
    }
}
