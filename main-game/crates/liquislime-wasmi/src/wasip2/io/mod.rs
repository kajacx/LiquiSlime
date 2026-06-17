use crate::{
    component_adaptor::ResultType,
    wasip2::{AddToLinkerParams, WasiP2TypesBuilder},
};
use waclay::*;

pub mod error;
pub mod poll;
pub mod streams;

#[allow(unused)]
pub struct IoResources {
    pub error: error::ErrorResources,
    pub poll: poll::PollResources,
    pub streams: streams::StreamResources,
}

pub fn register_resources(linker: &mut Linker) -> ResultType<IoResources> {
    let error = error::register_resources(linker)?;
    let poll = poll::register_resources(linker)?;
    let streams = streams::register_resources(linker)?;

    Ok(IoResources {
        error,
        poll,
        streams,
    })
}

pub fn register_types(builder: &mut WasiP2TypesBuilder) -> ResultType<()> {
    streams::register_types(builder)?;
    Ok(())
}

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    poll::add_to_linker(params)?;
    streams::add_to_linker(params)?;
    Ok(())
}
