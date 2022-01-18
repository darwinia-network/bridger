use microkv::namespace::NamespaceMicroKV;
use microkv::MicroKV;

pub fn migrate(microkv: &MicroKV) -> color_eyre::Result<()> {
    let n_microkv = microkv.namespace("task-darwinia-ethereum");
    migrate_scan_darwinia(&n_microkv)?;
    migrate_scan_ethereum(&n_microkv)?;
    Ok(())
}

fn migrate_scan_darwinia(microkv: &NamespaceMicroKV) -> color_eyre::Result<()> {
    let key = "last-tracked-darwinia-block";
    let block_darwinia: Option<u64> = microkv.get_as(key)?;
    if let Some(block) = block_darwinia {
        microkv.put("scan.darwinia.next", &block)?;
        microkv.delete(key)?;
    }
    Ok(())
}

fn migrate_scan_ethereum(microkv: &NamespaceMicroKV) -> color_eyre::Result<()> {
    let key = "last-redeemed";
    let block_ropsten: Option<u64> = microkv.get_as(key)?;
    if let Some(block) = block_ropsten {
        microkv.put("scan.ethereum.next", &block)?;
        microkv.delete(key)?;
    }
    Ok(())
}
