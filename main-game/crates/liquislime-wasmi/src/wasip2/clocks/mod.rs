use crate::{
    component_adaptor::ResultType,
    wasip2::{AddToLinkerParams, WasiP2TypesBuilder},
};

mod monotonic_clock;
mod wall_clock;

pub fn register_types(builder: &mut WasiP2TypesBuilder) -> ResultType<()> {
    wall_clock::register_types(builder)?;
    Ok(())
}

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    monotonic_clock::add_to_linker(params)?;
    wall_clock::add_to_linker(params)?;
    Ok(())
}
