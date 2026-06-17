use crate::{
    component_adaptor::ResultType,
    wasip2::{AddToLinkerParams, WasiP2TypesBuilder},
};
use waclay::*;

mod preopens;
mod types;

#[allow(unused)]
pub struct FilesystemResources {
    pub types: types::TypesResources,
}

pub fn register_resources(linker: &mut Linker) -> ResultType<FilesystemResources> {
    let types = types::register_resources(linker)?;

    Ok(FilesystemResources { types })
}
pub fn register_types(builder: &mut WasiP2TypesBuilder) -> ResultType<()> {
    types::register_types(builder)?;
    Ok(())
}

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    types::add_to_linker(params)?;
    preopens::add_to_linker(params)?;
    Ok(())
}
