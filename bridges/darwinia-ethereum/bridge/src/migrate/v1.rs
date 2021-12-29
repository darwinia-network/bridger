use microkv::namespace::NamespaceMicroKV;

use support_tracker::Tracker;

pub fn migrate(microkv: &NamespaceMicroKV) -> color_eyre::Result<()> {
    auto_start_scan(microkv)?;
    Ok(())
}

fn auto_start_scan(microkv: &NamespaceMicroKV) -> color_eyre::Result<()> {
    if microkv.get("scan.darwinia.next")?.is_some() {
        let tracker_darwinia = Tracker::new(microkv.clone(), "scan.darwinia");
        tracker_darwinia.start_running()?;
    }

    if microkv.get("scan.ethereum.next")?.is_some() {
        let tracker_ropsten = Tracker::new(microkv.clone(), "scan.ethereum");
        tracker_ropsten.start_running()?;
    }

    Ok(())
}
