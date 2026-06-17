use crate::{component_adaptor::ResultType, wasip2::AddToLinkerParams};
use waclay::*;

mod environment;
mod exit;
mod stderr;
mod stdin;
mod stdout;
mod terminal_input;
mod terminal_output;
mod terminal_stderr;
mod terminal_stdin;
mod terminal_stdout;

#[allow(unused)]
pub struct CliResources {
    pub terminal_input: terminal_input::TerminalInputResources,
    pub terminal_output: terminal_output::TerminalOutputResources,
}

pub fn register_resources(linker: &mut Linker) -> ResultType<CliResources> {
    let terminal_input = terminal_input::register_resources(linker)?;
    let terminal_output = terminal_output::register_resources(linker)?;

    Ok(CliResources {
        terminal_input,
        terminal_output,
    })
}

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    exit::add_to_linker(params)?;
    stdin::add_to_linker(params)?;
    stdout::add_to_linker(params)?;
    stderr::add_to_linker(params)?;
    environment::add_to_linker(params)?;
    terminal_stdin::add_to_linker(params)?;
    terminal_stdout::add_to_linker(params)?;
    terminal_stderr::add_to_linker(params)?;
    Ok(())
}
