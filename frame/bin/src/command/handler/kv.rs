use std::collections::HashMap;

use serde_json::Value;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

use component_state::state::BridgeState;
use microkv::namespace::NamespaceMicroKV;
use support_common::error::BridgerError;
use support_terminal::output;
use support_terminal::output::OutputFormat;

use crate::command::types::KvOpt;

pub fn handle_kv(namespace: Option<String>, opt: KvOpt) -> color_eyre::Result<()> {
    let namespace = namespace.unwrap_or_default();
    let state = BridgeState::new()?;
    match opt {
        KvOpt::Namespaces => handle_namespaces(&state),
        KvOpt::Put { kvs } => handle_put(&state, namespace, kvs),
        KvOpt::Get {
            keys,
            output,
            include_key,
        } => handle_get(&state, namespace, keys, output, include_key),
        KvOpt::Keys { sorted } => handle_keys(&state, namespace, sorted),
        KvOpt::Remove { keys } => handle_remove(&state, namespace, keys),
    }
}

fn handle_namespaces(state: &BridgeState) -> color_eyre::Result<()> {
    let microkv = state.microkv();
    let namespaces = microkv.namespaces()?;
    for ns in namespaces {
        output::output_text(ns);
    }
    Ok(())
}

fn handle_put(state: &BridgeState, namespace: String, kvs: Vec<String>) -> color_eyre::Result<()> {
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
    let microkv = state.microkv_with_namespace(namespace);

    let len = keys.len();
    for i in 0..len {
        let key = keys.get(i).unwrap();
        let value = values.get(i).unwrap();
        if key.is_empty() {
            continue;
        }
        spec_serialize_value(&microkv, key, value)?;
    }
    output::output_ok();
    Ok(())
}

fn handle_get(
    state: &BridgeState,
    namespace: String,
    keys: Vec<String>,
    output_format: OutputFormat,
    include_key: bool,
) -> color_eyre::Result<()> {
    let microkv = state.microkv_with_namespace(namespace);
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

fn handle_keys(state: &BridgeState, namespace: String, sorted: bool) -> color_eyre::Result<()> {
    let microkv = state.microkv_with_namespace(namespace);
    let keys = if sorted {
        microkv.sorted_keys()?
    } else {
        microkv.keys()?
    };
    keys.iter().for_each(output::output_text);
    Ok(())
}

fn handle_remove(
    state: &BridgeState,
    namespace: String,
    keys: Vec<String>,
) -> color_eyre::Result<()> {
    let microkv = state.microkv_with_namespace(namespace);
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
) -> color_eyre::Result<()> {
    let key = key.as_ref();
    let value = value.as_ref();

    let value = value.trim().to_string();
    if !key.contains("::") {
        microkv.put(key, &value)?;
        return Ok(());
    }
    let mut split = key.split("::").collect::<Vec<&str>>();
    let key = split.first().unwrap().to_string();
    split.remove(0);
    let value_type: String = split.join("::");
    match &value_type[..] {
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

pub fn best_view_option(value: &Option<Value>) -> color_eyre::Result<String> {
    match value {
        Some(v) => best_view(v),
        None => Ok("null".to_string()),
    }
}

pub fn best_view(value: &Value) -> color_eyre::Result<String> {
    if value.is_string() {
        return Ok(value.as_str().unwrap_or("").to_string());
    }
    Ok(serde_json::to_string_pretty(value)?)
}
