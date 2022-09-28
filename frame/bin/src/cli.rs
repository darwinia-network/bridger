use crate::command::handler;
use crate::Opt;

pub fn execute(opt: Opt) -> color_eyre::Result<()> {
    run(opt)
}

fn run(opt: Opt) -> color_eyre::Result<()> {
    match opt {
        Opt::List => handler::exec_list(),
        Opt::Registry { command } => handler::handle_registry(command),
    }
}
