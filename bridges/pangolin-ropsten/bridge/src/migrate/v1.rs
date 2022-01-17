use microkv::namespace::NamespaceMicroKV;
use microkv::MicroKV;

use support_tracker::Tracker;

pub fn migrate(microkv: &MicroKV) -> color_eyre::Result<()> {
    let n_microkv = microkv.namespace("task-pangolin-ropsten");
    auto_start_scan(&n_microkv)?;
    Ok(())
}

fn auto_start_scan(microkv: &NamespaceMicroKV) -> color_eyre::Result<()> {
    if microkv.get("scan.pangolin.next")?.is_some() {
        let tracker_pangolin = Tracker::new(microkv.clone(), "scan.pangolin");
        tracker_pangolin.start_running()?;
    }

    if microkv.get("scan.ropsten.next")?.is_some() {
        let tracker_ropsten = Tracker::new(microkv.clone(), "scan.ropsten");
        tracker_ropsten.start_running()?;
    }

    Ok(())
}
