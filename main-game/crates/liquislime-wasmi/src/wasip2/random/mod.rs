use crate::{component_adaptor::ResultType, wasip2::AddToLinkerParams};

mod random;

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    random::add_to_linker(params)?;
    Ok(())
}

pub use random::LoopingRng;
