use microkv::namespace::NamespaceMicroKV;

use support_tracker::Tracker;

use crate::bridge::PangolinRopstenTask;

pub fn migrate(microkv: &NamespaceMicroKV) -> color_eyre::Result<()> {
    auto_start_scan(&microkv)?;
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
