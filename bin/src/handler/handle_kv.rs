use std::collections::HashMap;

use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

use crate::patch;
use crate::types::command::KvCommand;
use crate::types::server::Resp;
use crate::types::transfer::{KvListParam, KvOperationParam};

const DEFAULT_NAMESPACE_VIEW: &str = "<DEFAULT>";

pub async fn handle_kv(
    server: String,
    namespace: Option<String>,
    command: KvCommand,
) -> anyhow::Result<()> {
    match command {
        KvCommand::Namespaces => {
            if namespace.is_some() {
                println!(
                    "{}",
                    namespace.unwrap_or_else(|| DEFAULT_NAMESPACE_VIEW.to_string())
                );
                return Ok(());
            }
            let resp = reqwest::Client::builder()
                .build()?
                .post(format!("{}/kv/namespaces", server))
                .send()
                .await?
                .json::<Resp<Vec<String>>>()
                .await?;
            if resp.is_err() {
                eprintln!("{}", resp.msg());
                return Ok(());
            }
            match resp.data() {
                Some(ns) => {
                    for item in ns {
                        if item.is_empty() {
                            println!("{}", DEFAULT_NAMESPACE_VIEW);
                        } else {
                            println!("{}", item);
                        }
                    }
                }
                None => {
                    println!("Not have any namespace");
                }
            }
        }
        KvCommand::Put { kvs } => {
            let mut keys = vec![];
            let mut values = vec![];
            for (ix, value) in kvs.iter().enumerate() {
                if (ix + 1) % 2 == 0 {
                    values.push(value.clone());
                } else {
                    keys.push(value.clone());
                }
            }
            if keys.len() != values.len() {
                println!("The Key-Value length not same");
                return Ok(());
            }
            let param = KvOperationParam {
                namespace,
                keys,
                values,
            };
            let resp = reqwest::Client::builder()
                .build()?
                .post(format!("{}/kv/put", server))
                .json(&param)
                .send()
                .await?
                .json::<Resp<String>>()
                .await?;
            if resp.is_err() {
                eprintln!("{}", resp.msg());
                return Ok(());
            }
            println!("Success");
        }
        KvCommand::Get {
            keys,
            output,
            include_key,
        } => {
            let param = KvOperationParam {
                namespace,
                keys: keys.clone(),
                values: vec![],
            };
            let resp = reqwest::Client::builder()
                .build()?
                .post(format!("{}/kv/get", server))
                .json(&param)
                .send()
                .await?
                .json::<Resp<Vec<serde_json::Value>>>()
                .await?;
            if resp.is_err() {
                eprintln!("{}", resp.msg());
                return Ok(());
            }
            match resp.data() {
                Some(values) => {
                    let output = output.to_lowercase();
                    match &output[..] {
                        "json" => {
                            if include_key {
                                let mut map = HashMap::new();
                                for (ix, value) in values.iter().enumerate() {
                                    let key = keys.get(ix).unwrap();
                                    map.insert(key.to_string(), value.clone());
                                }
                                let json = serde_json::to_string_pretty(&map)?;
                                println!("{}", json);
                            } else {
                                let json = serde_json::to_string_pretty(&values)?;
                                println!("{}", json);
                            }
                        }
                        "table" => {
                            let mut table = Table::new();
                            table.max_column_width = 40;
                            table.style = TableStyle::simple();
                            let len = keys.len();
                            for ix in 0..len {
                                let value = values.get(ix);
                                let view = patch::helpers::best_view_option(value)?;
                                if include_key {
                                    let key = keys.get(ix).unwrap();
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
                            println!("{}", table.render());
                        }
                        _ => {
                            if include_key {
                                let len = keys.len();
                                for ix in 0..len {
                                    let key = keys.get(ix).unwrap();
                                    let value = values.get(ix);
                                    let view = patch::helpers::best_view_option(value)?;
                                    println!("{}", key);
                                    println!("{}", view);
                                }
                            } else {
                                for item in values {
                                    let view = patch::helpers::best_view(item)?;
                                    println!("{}", view);
                                }
                            }
                        }
                    }
                }
                None => {
                    println!("Not found these keys");
                }
            }
        }
        KvCommand::List { sorted } => {
            let param = KvListParam { namespace, sorted };
            let resp = reqwest::Client::builder()
                .build()?
                .post(format!("{}/kv/list", server))
                .json(&param)
                .send()
                .await?
                .json::<Resp<Vec<String>>>()
                .await?;
            if resp.is_err() {
                eprintln!("{}", resp.msg());
                return Ok(());
            }
            match resp.data() {
                Some(data) => data.iter().for_each(|key| println!("{}", key)),
                None => println!("Not have any keys"),
            }
        }
        KvCommand::Remove { keys } => {
            let param = KvOperationParam {
                namespace,
                keys,
                values: vec![],
            };
            let resp = reqwest::Client::builder()
                .build()?
                .post(format!("{}/kv/remove", server))
                .json(&param)
                .send()
                .await?
                .json::<Resp<String>>()
                .await?;
            if resp.is_err() {
                eprintln!("{}", resp.msg());
                return Ok(());
            }
            println!("Success");
        }
    }
    Ok(())
}
