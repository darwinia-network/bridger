use client_pangolin::types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel;
use component_shadow::component::ShadowComponent;
use support_common::config::{Config, Names};
use support_terminal::output;
use support_terminal::output::OutputFormat;
use support_terminal::types::BasicOptions;

use crate::bridge::PangolinRopstenConfig;
use crate::command::types::ParcelOpts;

pub async fn handle_parcel(opts: ParcelOpts, basic: BasicOptions) -> color_eyre::Result<()> {
    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;
    let output_format = basic.output;
    let block = opts.block;

    let shadow = ShadowComponent::component(
        bridge_config.shadow,
        bridge_config.ethereum,
        bridge_config.web3,
    )?;

    // Get parcel
    let parcel = shadow.parcel(block as usize).await?;
    let parcel: EthereumRelayHeaderParcel = parcel.try_into()?;
    let text = match output_format {
        OutputFormat::Json => serde_json::to_string(&parcel)?,
        _ => format!("{:?}", parcel),
    };
    output::output_text(text);
    Ok(())
}
