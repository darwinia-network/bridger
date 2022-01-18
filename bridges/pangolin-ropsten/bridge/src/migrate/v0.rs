use microkv::namespace::NamespaceMicroKV;
use microkv::MicroKV;

pub fn migrate(microkv: &MicroKV) -> color_eyre::Result<()> {
    let n_microkv = microkv.namespace("task-pangolin-ropsten");
    migrate_scan_pangolin(&n_microkv)?;
    migrate_scan_opsten(&n_microkv)?;
    Ok(())
}

fn migrate_scan_pangolin(microkv: &NamespaceMicroKV) -> color_eyre::Result<()> {
    let key = "last-tracked-pangolin-block";
    let block_pangolin: Option<u64> = microkv.get_as(key)?;
    if let Some(block) = block_pangolin {
        microkv.put("scan.pangolin.next", &block)?;
        microkv.delete(key)?;
    }
    Ok(())
}

fn migrate_scan_opsten(microkv: &NamespaceMicroKV) -> color_eyre::Result<()> {
    let key = "last-redeemed-ropsten";
    let block_ropsten: Option<u64> = microkv.get_as(key)?;
    if let Some(block) = block_ropsten {
        microkv.put("scan.ropsten.next", &block)?;
        microkv.delete(key)?;
    }
    Ok(())
}
