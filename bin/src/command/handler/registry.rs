use colored::Colorize;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

use support_common::config::{Config, Names};
use support_terminal::output::{self, OutputFormat};

use crate::command::types::{RegistryOpt, RegistryType};
use crate::config::BridgerConfig;

pub fn handle_registry(opt: RegistryOpt) -> color_eyre::Result<()> {
    match opt {
        RegistryOpt::Set { type_, path } => handle_set(type_, path),
        RegistryOpt::Get { output } => handle_get(output),
    }
}

fn handle_set(type_: RegistryType, mut path: Option<String>) -> color_eyre::Result<()> {
    if type_ == RegistryType::Github && path.is_none() {
        path = Some("https://github.com/darwinia-network/bridger".to_string());
    }
    if type_ != RegistryType::Local && path.is_none() {
        output::output_err_and_exit("Please provide `--path <path>`");
    }
    let mut config: BridgerConfig = Config::restore(Names::Bridger)?;
    tracing::trace!(
        target: "bridger",
        "Set registry [{}]{}",
        format!("{:?}", type_).cyan(),
        if let Some(v) = &path { format!(": {}", v) } else { Default::default() }
    );
    config.registry.type_ = type_;
    config.registry.path = path;
    Config::store(Names::Bridger, config)?;
    output::output_ok();
    Ok(())
}

fn handle_get(out: OutputFormat) -> color_eyre::Result<()> {
    let config: BridgerConfig = Config::restore(Names::Bridger)?;
    match out {
        OutputFormat::Raw => {
            let type_ = config.registry.type_;
            output::output_text(format!("{}: {:?}", "TYPE".bold(), &type_));
            if type_ != RegistryType::Local {
                output::output_text(format!(
                    "{}: {}",
                    "PATH".bold(),
                    config.registry.path.unwrap_or_default()
                ));
            }
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&config.registry)?;
            output::output_text(json);
        }
        OutputFormat::Table => {
            let registry = config.registry;
            let mut table = Table::new();
            table.max_column_width = 40;
            table.separate_rows = false;
            table.style = TableStyle::empty();
            table.add_row(Row::new(vec![
                TableCell::new("Type".bold()),
                TableCell::new("Path".bold()),
            ]));
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(format!("{:?}", registry.type_), 2, Alignment::Left),
                TableCell::new_with_alignment(
                    registry.path.unwrap_or_default(),
                    2,
                    Alignment::Left,
                ),
            ]));
            output::output_text(table.render());
        }
    }
    Ok(())
}
