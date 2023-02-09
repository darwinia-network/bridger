#![allow(dead_code)]

use std::process;

use colored::Colorize;

use crate::output::OutputFormat;

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

pub fn output_text(text: impl AsRef<str>) {
    println!("{}", text.as_ref());
}

pub fn unsupport_output_format(output: OutputFormat) {
    let outf = format!("{output:?}");
    println!("Not support this format: [{}]", &outf[..].red());
}

pub fn output_warning(text: impl AsRef<str>) {
    println!("⚠️{}", text.as_ref().yellow());
}
