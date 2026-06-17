use crate::component_adaptor::ResultType;
use waclay::*;

#[allow(unused)]
pub struct TerminalInputResources {
    pub terminal_input: ResourceType,
}

pub struct TerminalInputResource {}

pub fn register_resources(linker: &mut Linker) -> ResultType<TerminalInputResources> {
    let instance = linker.define_instance("wasi:cli/terminal-input@0.2.6".try_into()?)?;

    let terminal_input = ResourceType::new::<TerminalInputResource>(None);
    instance.define_resource("terminal-input", terminal_input.clone())?;

    Ok(TerminalInputResources { terminal_input })
}
