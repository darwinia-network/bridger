use shadow_liketh::component::ShadowComponent;
use shadow_liketh::types::BridgeName;
use support_common::config::{Config, Names};
use support_terminal::output;
use support_terminal::output::OutputFormat;
use support_terminal::types::BasicOptions;

use crate::bridge::DarwiniaEthereumConfig;
use crate::command::types::ParcelOpts;

pub async fn handle_parcel(opts: ParcelOpts, basic: BasicOptions) -> color_eyre::Result<()> {
    let bridge_config: DarwiniaEthereumConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;
    let output_format = basic.output;
    let block = opts.block;

    let shadow = ShadowComponent::component(
        bridge_config.shadow,
        bridge_config.ethereum,
        bridge_config.web3,
        BridgeName::DarwiniaEthereum,
    )?;

    // Get parcel
    let parcel = shadow.parcel(block as u64).await?;
    let text = match output_format {
        OutputFormat::Json => serde_json::to_string(&parcel)?,
        _ => format!("{:?}", parcel),
    };
    output::output_text(text);
    Ok(())
}
