use std::collections::HashMap;

use microkv::namespace::NamespaceMicroKV;
use microkv::MicroKV;
use serde_json::Value;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

use component_state::state::{BridgeState, StateOptions};
use support_common::error::BridgerError;
use support_terminal::output;
use support_terminal::output::OutputFormat;

use crate::types::KvOpts;

/// Handle kv command
pub fn handle_kv(
    state_option: StateOptions,
    namespace: Option<String>,
    opt: KvOpts,
) -> color_eyre::Result<()> {
    let namespace = namespace.unwrap_or_default();
    let state = BridgeState::new(state_option)?;
    let microkv = state.microkv_with_namespace(&namespace);
    match opt {
        KvOpts::Namespaces => handle_namespaces(state.microkv()),
        KvOpts::Put { kvs } => handle_put(&microkv, kvs, &namespace),
        KvOpts::Get {
            keys,
            output,
            include_key,
        } => handle_get(&microkv, keys, output, include_key),
        KvOpts::Keys { sorted } => handle_keys(&microkv, sorted),
        KvOpts::Remove { keys } => handle_remove(&microkv, keys),
    }
}

fn handle_namespaces(microkv: &MicroKV) -> color_eyre::Result<()> {
    let namespaces = microkv.namespaces()?;
    for ns in namespaces {
        output::output_text(ns);
    }
    Ok(())
}

fn handle_put(microkv: &NamespaceMicroKV, kvs: Vec<String>, namespace: &str) -> color_eyre::Result<()> {
    let mut keys = Vec::new();
    let mut values = Vec::new();
    for (ix, value) in kvs.iter().enumerate() {
        if (ix + 1) % 2 == 0 {
            values.push(value.clone());
        } else {
            keys.push(value.clone());
        }
    }
    if keys.len() != values.len() {
        output::output_err_and_exit("The Key-Value length not same");
    }

    let len = keys.len();
    for i in 0..len {
        let key = keys.get(i).unwrap();
        let value = values.get(i).unwrap();
        if key.is_empty() {
            continue;
        }
        let value_type: Option<String> = try_get_value_type(namespace, key);
        spec_serialize_value(microkv, key, value, value_type)?;
    }
    output::output_ok();
    Ok(())
}

fn try_get_value_type(namespace: &str, key: &str) -> Option<String> {
    let schema = include_str!("schema.toml");
    if let toml::Value::Table(table) = schema.parse::<toml::Value>().unwrap() {
        if let Some(toml::Value::Table(ns)) = table.get(namespace) {
            return ns
                .get(key)
                .map(|value| value.as_str().unwrap().to_owned() )
        }
    }
    None
}

fn handle_get(
    microkv: &NamespaceMicroKV,
    keys: Vec<String>,
    output_format: OutputFormat,
    include_key: bool,
) -> color_eyre::Result<()> {
    let mut kvs = Vec::new();
    for key in keys {
        let value = microkv.get(&key)?;
        kvs.push((key, value));
    }

    match output_format {
        OutputFormat::Raw => {
            if include_key {
                let len = kvs.len();
                for ix in 0..len {
                    let (key, value) = kvs.get(ix).unwrap();
                    let view = best_view_option(value)?;
                    output::output_text(key);
                    output::output_text(view);
                }
            } else {
                for (_key, value) in kvs {
                    let view = best_view_option(&value)?;
                    output::output_text(view);
                }
            }
        }
        OutputFormat::Json => {
            if include_key {
                let mut map = HashMap::new();
                for (_ix, (key, value)) in kvs.iter().enumerate() {
                    map.insert(key.to_string(), value.clone());
                }
                let json = serde_json::to_string_pretty(&map)?;
                output::output_text(json);
            } else {
                let values = kvs
                    .iter()
                    .map(|(_key, value)| value)
                    .collect::<Vec<&Option<Value>>>();
                let json = serde_json::to_string_pretty(&values)?;
                output::output_text(json);
            }
        }
        OutputFormat::Table => {
            let mut table = Table::new();
            table.max_column_width = 40;
            table.style = TableStyle::simple();
            for (key, value) in kvs {
                let view = best_view_option(&value)?;
                if include_key {
                    table.add_row(Row::new(vec![
                        TableCell::new_with_alignment(key, 1, Alignment::Left),
                        TableCell::new_with_alignment(view, 1, Alignment::Left),
                    ]));
                } else {
                    table.add_row(Row::new(vec![TableCell::new_with_alignment(
                        view,
                        1,
                        Alignment::Left,
                    )]));
                }
            }
            output::output_text(table.render());
        }
    }
    Ok(())
}

fn handle_keys(microkv: &NamespaceMicroKV, sorted: bool) -> color_eyre::Result<()> {
    let keys = if sorted {
        microkv.sorted_keys()?
    } else {
        microkv.keys()?
    };
    keys.iter().for_each(output::output_text);
    Ok(())
}

fn handle_remove(microkv: &NamespaceMicroKV, keys: Vec<String>) -> color_eyre::Result<()> {
    for key in keys {
        microkv.delete(key)?;
    }
    output::output_ok();
    Ok(())
}

// kv serialize value.
// todo: there need change to schema https://github.com/darwinia-network/bridger/issues/377
fn spec_serialize_value(
    microkv: &NamespaceMicroKV,
    key: impl AsRef<str>,
    value: impl AsRef<str>,
    value_type: Option<impl AsRef<str>>
) -> color_eyre::Result<()> {
    let key = key.as_ref();
    let value = value.as_ref();

    let value = value.trim().to_string();
    let value_type = match value_type {
        None => {
            output::output_warning(format!("Schema of '{}' not found, use String as default", key));
            "String".to_owned()
        },
        Some(v) => v.as_ref().to_owned()
    };

    match value_type.as_ref() {
        "String" | "string" | "str" => {
            microkv.put(key, &value)?;
        }
        "isize" | "i8" | "i16" | "i32" | "i64" | "i128" => {
            let value = value.parse::<isize>()?;
            microkv.put(key, &value)?;
        }
        "usize" | "u8" | "u16" | "u32" | "u64" | "u128" => {
            let value = value.parse::<usize>()?;
            microkv.put(key, &value)?;
        }
        "f32" | "f64" => {
            let value = value.parse::<f64>()?;
            microkv.put(key, &value)?;
        }
        "bool" => {
            let value = value.parse::<bool>()?;
            microkv.put(key, &value)?;
        }
        _ => {
            return Err(
                BridgerError::Custom(format!("Not support value type: {}", value_type)).into(),
            );
        }
    }
    Ok(())
}

fn best_view_option(value: &Option<Value>) -> color_eyre::Result<String> {
    match value {
        Some(v) => best_view(v),
        None => Ok("null".to_string()),
    }
}

fn best_view(value: &Value) -> color_eyre::Result<String> {
    if value.is_string() {
        return Ok(value.as_str().unwrap_or("").to_string());
    }
    Ok(serde_json::to_string_pretty(value)?)
}
