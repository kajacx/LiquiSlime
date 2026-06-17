use crate::component_adaptor::ResultType;
use waclay::*;

#[allow(unused)]
pub struct ErrorResources {
    pub error: ResourceType,
}

struct ErrorResource {}

pub fn register_resources(linker: &mut Linker) -> ResultType<ErrorResources> {
    let instance = linker.define_instance("wasi:io/error@0.2.6".try_into()?)?;

    let error = ResourceType::new::<ErrorResource>(None);
    instance.define_resource("error", error.clone())?;

    Ok(ErrorResources { error })
}
