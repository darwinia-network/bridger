use microkv::namespace::NamespaceMicroKV;
use microkv::MicroKV;

use support_common::error::BridgerError;

mod v0;
mod v1;
mod v2;
mod v3;

pub fn migrate(microkv: &MicroKV, version: usize) -> color_eyre::Result<()> {
    let saved_version = current_version(microkv)?;
    // same version, no migrate
    if saved_version == version {
        return Ok(());
    }

    let steps: Vec<Box<dyn Fn(&MicroKV) -> color_eyre::Result<()>>> = vec![
        Box::new(v0::migrate),
        Box::new(v1::migrate),
        Box::new(v2::migrate),
        Box::new(v3::migrate),
    ];

    let max_version = steps.len() - 1;
    if version > max_version {
        return Err(BridgerError::Migration(format!(
            "Support max version: {}, but want upgrade to {}.",
            max_version, version
        ))
        .into());
    }
    let from = if saved_version == 0 {
        0
    } else {
        saved_version + 1
    };
    let to = version + 1;
    for ix in from..to {
        let migration = steps.get(ix).unwrap();
        if let Err(e) = migration(microkv) {
            return Err(BridgerError::Migration(format!(
                "Failed to migrate. step [{}]: {:?}",
                ix, e
            ))
            .into());
        }
    }
    flush_version(microkv, version)?;
    Ok(())
}

fn current_version(microkv: &MicroKV) -> color_eyre::Result<usize> {
    let n_microkv = better_namespace_kv(microkv)?;
    let version: Option<usize> = n_microkv.get_as(".version")?;
    Ok(version.unwrap_or(0))
}

fn flush_version(microkv: &MicroKV, version: usize) -> color_eyre::Result<()> {
    let n_microkv = better_namespace_kv(microkv)?;
    n_microkv.put(".version", &version)?;
    Ok(())
}

fn better_namespace_kv(microkv: &MicroKV) -> color_eyre::Result<NamespaceMicroKV> {
    let nss = microkv.namespaces()?;
    if let Some(old) = nss.iter().find(|&ns| &ns[..] == "task-darwinia-ethereum") {
        Ok(microkv.namespace(old))
    } else {
        Ok(microkv.namespace("darwinia-ethereum"))
    }
}
