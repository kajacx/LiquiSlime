use crate::component_adaptor::ResultType;
use waclay::*;

#[allow(unused)]
pub struct TerminalOutputResources {
    pub terminal_output: ResourceType,
}

pub struct TerminalOutputResource {}

pub fn register_resources(linker: &mut Linker) -> ResultType<TerminalOutputResources> {
    let instance = linker.define_instance("wasi:cli/terminal-output@0.2.6".try_into()?)?;

    let terminal_output = ResourceType::new::<TerminalOutputResource>(None);
    instance.define_resource("terminal-output", terminal_output.clone())?;

    Ok(TerminalOutputResources { terminal_output })
}
