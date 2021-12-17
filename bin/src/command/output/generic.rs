use std::process;

use colored::Colorize;

pub fn output_ok() {
    println!("{}", "Success".green())
}

pub fn output_err_and_exit(msg: impl AsRef<str>) -> ! {
    output_err(msg);
    process::exit(1);
}

pub fn output_err(msg: impl AsRef<str>) {
    eprintln!("{}", msg.as_ref().red());
}
