use microkv::MicroKV;

pub fn migrate(microkv: &MicroKV) -> color_eyre::Result<()> {
    let old_microkv = microkv.namespace("task-darwinia-ethereum");
    let new_microkv = microkv.namespace("darwinia-ethereum");
    let keys = old_microkv.keys()?;
    for key in keys {
        if let Some(v) = old_microkv.get(&key)? {
            new_microkv.put(key, &v)?;
        }
    }
    microkv.delete_namespace("task-darwinia-ethereum")?;
    Ok(())
}
