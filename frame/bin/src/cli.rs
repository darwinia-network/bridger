use crate::command::handler;
use crate::Opt;

pub fn execute(opt: Opt) -> color_eyre::Result<()> {
    if let Err(e) = run(opt) {
        // maybe there have some special error to handle.
        return Err(e);
    }
    Ok(())
}

fn run(opt: Opt) -> color_eyre::Result<()> {
    match opt {
        Opt::List => handler::exec_list(),
        Opt::Registry { command } => handler::handle_registry(command),
        Opt::Kv { namespace, command } => handler::handle_kv(namespace, command),
    }
}
