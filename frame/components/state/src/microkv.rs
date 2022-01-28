use crate::config::MicrokvConfig;
use crate::error::StateComponentError;

pub fn microkv_instance(config: &MicrokvConfig) -> color_eyre::Result<microkv::MicroKV> {
    let dbname = config
        .db_name
        .clone()
        .ok_or_else(|| StateComponentError::Microkv("Missing microkv db name".to_string()))?;

    let mut microkv = try_microkv(dbname, config)?;
    microkv = microkv.set_auto_commit(config.auto_commit);
    microkv = microkv.set_enable_reload();
    microkv.commit()?;
    Ok(microkv)
}

fn try_microkv(dbname: String, config: &MicrokvConfig) -> color_eyre::Result<microkv::MicroKV> {
    Ok(microkv::MicroKV::open_with_base_path(
        dbname.clone(),
        config.base_path.clone(),
    )?)
}
