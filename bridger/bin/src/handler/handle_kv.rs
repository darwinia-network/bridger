use crate::types::command::KvCommand;
use crate::types::server::Resp;
use crate::types::transfer::{KvListParam, KvOperationParam};

pub async fn handle_kv(
    server: String,
    namespace: Option<String>,
    command: KvCommand,
) -> anyhow::Result<()> {
    match command {
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
        KvCommand::Get { keys } => {
            let param = KvOperationParam {
                namespace,
                keys,
                values: vec![],
            };
            let resp = reqwest::Client::builder()
                .build()?
                .post(format!("{}/kv/get", server))
                .json(&param)
                .send()
                .await?
                .json::<Resp<serde_json::Value>>()
                .await?;
            if resp.is_err() {
                eprintln!("{}", resp.msg());
                return Ok(());
            }
            match resp.data() {
                Some(data) => {
                    let output = serde_json::to_string_pretty(&data)?;
                    println!("{}", output);
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
