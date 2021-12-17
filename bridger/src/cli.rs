use crate::command::handler;
use crate::Opt;

pub fn execute(opt: Opt) -> color_eyre::Result<()> {
    if let Err(_) = run(opt) {
        // maybe there have some special error to handle.
    }
    Ok(())
}

fn run(opt: Opt) -> color_eyre::Result<()> {
    match opt {
        Opt::List => handler::exec_list(),
        Opt::Registry { type_, path } => handler::handle_registry(type_, path),
    }
}
