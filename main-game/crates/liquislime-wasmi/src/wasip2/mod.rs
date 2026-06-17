use crate::{
    component_adaptor::{ResultType, StoreType},
    wasip2::type_builder::WasiP2Types,
};
use waclay::*;

pub mod cli;
pub mod clocks;
pub mod filesystem;
pub mod io;
pub mod random;
pub mod type_builder;

use type_builder::WasiP2TypesBuilder;

#[allow(unused)]
pub struct WasiP2Resources {
    cli: cli::CliResources,
    filesystem: filesystem::FilesystemResources,
    io: io::IoResources,
}

fn register_resources(linker: &mut Linker) -> ResultType<WasiP2Resources> {
    let cli = cli::register_resources(linker)?;
    let filesystem = filesystem::register_resources(linker)?;
    let io = io::register_resources(linker)?;

    Ok(WasiP2Resources {
        cli,
        filesystem,
        io,
    })
}

fn register_types(builder: &mut WasiP2TypesBuilder) -> ResultType<()> {
    clocks::register_types(builder)?;
    filesystem::register_types(builder)?;
    io::register_types(builder)?;
    Ok(())
}

pub struct AddToLinkerParams<'a> {
    store: &'a mut StoreType,
    linker: &'a mut Linker,
    resources: WasiP2Resources,
    types: WasiP2Types,
}

pub fn add_to_linker(store: &mut StoreType, linker: &mut Linker) -> ResultType<()> {
    let resources = register_resources(linker)?;

    let mut builder = WasiP2TypesBuilder::default();
    register_types(&mut builder)?;
    let types = builder.build(&resources)?;

    let mut params = AddToLinkerParams {
        store,
        linker,
        resources,
        types,
    };

    cli::add_to_linker(&mut params)?;
    filesystem::add_to_linker(&mut params)?;
    io::add_to_linker(&mut params)?;
    clocks::add_to_linker(&mut params)?;
    random::add_to_linker(&mut params)?;

    Ok(())
}

pub use random::LoopingRng;
