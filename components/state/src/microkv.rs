use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use bridge_traits::bridge::config::BridgeConfig;
use bridge_traits::error::StandardError;

use crate::config::MicrokvConfig;

pub fn microkv_instance(config: &MicrokvConfig) -> anyhow::Result<microkv::MicroKV> {
    let dbname = config
        .db_name
        .clone()
        .unwrap_or_else(|| MicrokvConfig::marker().to_string());

    let microkv = try_microkv(dbname, config)?;
    microkv.commit()?;
    Ok(microkv)
}

fn try_microkv(dbname: String, config: &MicrokvConfig) -> anyhow::Result<microkv::MicroKV> {
    match microkv::MicroKV::open_with_base_path(dbname.clone(), config.base_path.clone()) {
        Ok(kv) => Ok(kv),
        Err(e) => match e.error {
            microkv::errors::ErrorType::MigrateError(from, to) => {
                if &from == "<0.3.0" && to.starts_with("0.3.") {
                    return try_microkv_migrate_less030_to_03x(dbname, config);
                }
                Err(StandardError::Component(format!("Failed migrate microkv, {:?}", e.msg)).into())
            }
            _ => Err(StandardError::Component(format!(
                "Failed create microkv instance: {:?}",
                e.msg
            ))
            .into()),
        },
    }
}

fn try_microkv_migrate_less030_to_03x(
    dbname: String,
    config: &MicrokvConfig,
) -> anyhow::Result<microkv::history::MicroKV030> {
    let key_relayed = "relayed".to_string();
    let key_target = "target".to_string();
    let key_last_tracked_darwinia_block = "last-tracked-darwinia-block".to_string();
    let key_last_tracked_pangolin_block = "last-tracked-pangolin-block".to_string();
    let key_last_redeemed = "last-redeemed".to_string();
    let key_last_redeemed_ropsten = "last-redeemed-ropsten".to_string();
    let key_password_darwinia_ethereum = "task-darwinia-ethereum@password".to_string();

    let path_db = microkv::helpers::get_db_path_with_base_path(dbname, config.base_path.clone());
    let less030: microkv::history::MicroKVLess030 =
        microkv::helpers::read_file_and_deserialize_bincode(&path_db)?;
    let relayed: Option<u64> = less030.lock_read(|kv| {
        kv.get(&key_relayed)
            .and_then(|v| less030.decode_value(&v).ok())
    })?;
    let target: Option<u64> = less030.lock_read(|kv| {
        kv.get(&key_target)
            .and_then(|v| less030.decode_value(&v).ok())
    })?;
    let last_tracked_darwinia_block: Option<u32> = less030.lock_read(|kv| {
        kv.get(&key_last_tracked_darwinia_block)
            .and_then(|v| less030.decode_value(&v).ok())
    })?;
    let last_tracked_pangolin_block: Option<u32> = less030.lock_read(|kv| {
        kv.get(&key_last_tracked_pangolin_block)
            .and_then(|v| less030.decode_value(&v).ok())
    })?;
    let last_redeemed: Option<u64> = less030.lock_read(|kv| {
        kv.get(&key_last_redeemed)
            .and_then(|v| less030.decode_value(&v).ok())
    })?;
    let last_redeemed_ropsten: Option<u64> = less030.lock_read(|kv| {
        kv.get(&key_last_redeemed_ropsten)
            .and_then(|v| less030.decode_value(&v).ok())
    })?;
    let password_darwinia_ethereum: Option<u64> = less030.lock_read(|kv| {
        kv.get(&key_password_darwinia_ethereum)
            .and_then(|v| less030.decode_value(&v).ok())
    })?;

    // create 0.3.x microkv instance

    let nonce = microkv::helpers::gen_nonce();
    let storage = Arc::new(RwLock::new(HashMap::new()));

    let microkv_030 =
        microkv::history::MicroKV030::create(path_db, None, nonce, config.auto_commit, storage);
    // migrate task-darwinia-ethereum
    let kv_darwinia_ethereum = microkv_030.namespace("task-darwinia-ethereum");
    relayed.map(|value| kv_darwinia_ethereum.put(&key_relayed, &value));
    target.map(|value| kv_darwinia_ethereum.put(&key_target, &value));
    last_tracked_darwinia_block
        .map(|value| kv_darwinia_ethereum.put(&key_last_tracked_darwinia_block, &value));
    last_redeemed.map(|value| kv_darwinia_ethereum.put(&key_last_redeemed, &value));

    // migrate task-pangolin-ropsten
    let kv_pangolin_eopsten = microkv_030.namespace("task-pangolin-ropsten");
    relayed.map(|value| kv_pangolin_eopsten.put(&key_relayed, &value));
    target.map(|value| kv_pangolin_eopsten.put(&key_target, &value));
    last_tracked_pangolin_block
        .map(|value| kv_pangolin_eopsten.put(&key_last_tracked_pangolin_block, &value));
    last_redeemed_ropsten.map(|value| kv_pangolin_eopsten.put(&key_last_redeemed_ropsten, &value));

    // bridger security
    let kv_security = microkv_030.namespace(crate::state::NS_SECURITY);
    password_darwinia_ethereum
        .map(|value| kv_security.put(&key_password_darwinia_ethereum, &value));

    Ok(microkv_030)
}
