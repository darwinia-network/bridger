use codec::Encode;
use colored::*;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::task::TaskTerminal;
use bridge_traits::error::StandardError;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::events::EventInfo;
use component_darwinia_subxt::rpc::FormatedMMR;
use component_darwinia_subxt::to_ethereum::Darwinia2Ethereum;

use crate::bus::DarwiniaEthereumBus;
use crate::task::DarwiniaEthereumTask;

#[derive(Default, Debug)]
struct TxProofWithMMRProof {
    message: String,
    signatures: Vec<String>,
    root: String,
    mmrindex: u64,
    block_header: String,
    peaks: Vec<String>,
    siblings: Vec<String>,
    events_proof_str: String,
    signers: Vec<String>,
}

impl std::fmt::Display for TxProofWithMMRProof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            r#"{{
    message: {},
    signatures: [{}],
    root: {},
    MMRIndex: {},
    blockNumber: {},
    peaks: [{}],
    siblings: [{}],
    eventsProofStr: {},
    signers: {}
}}"#,
            self.message.green(),
            self.signatures.join(",").green(),
            self.root.green(),
            self.mmrindex.to_string().red(),
            self.block_header.green(),
            self.peaks.join(",").green(),
            self.siblings.join(",").green(),
            self.events_proof_str.green(),
            self.signers.join(","),
        )
    }
}

pub async fn d2e(
    _bus: &DarwiniaEthereumBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let network = param["network"]
        .as_str()
        .ok_or_else(|| StandardError::Api("The `network` parameter is required".to_string()))?;
    let txblock = param["txblock"]
        .as_u64()
        .ok_or_else(|| StandardError::Api("The `txblock` parameter is required".to_string()))?;
    let mmrblock = param["mmrblock"]
        .as_u64()
        .ok_or_else(|| StandardError::Api("The `mmrblock` parameter is required".to_string()))?;
    let signblock = param["signblock"]
        .as_u64()
        .ok_or_else(|| StandardError::Api("The `signblock` parameter is required".to_string()))?;

    // Darwinia client
    let component_darwinia = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
    let darwinia = component_darwinia.component().await?;
    let darwinia_to_ethereum = Darwinia2Ethereum::new(darwinia.clone());

    // mmr root block
    let mmr_root = darwinia.get_mmr_root(mmrblock as u32).await?;
    let message = Darwinia2Ethereum::construct_mmr_root_message(
        network.to_string(),
        mmrblock as u32,
        mmr_root,
    );

    let header = darwinia.get_block_by_number(txblock as u32).await?;
    let proof = darwinia_to_ethereum
        .get_headermmr_genproof(txblock, mmrblock, header.hash())
        .await?;
    let event_proof = darwinia
        .get_event_proof(
            array_bytes::hex2bytes(
                "f8860dda3d08046cf2706b92bf7202eaae7a79191c90e76297e0895605b8b457",
            )
            .unwrap(),
            header.hash(),
        )
        .await?;

    //let mut result = HashMap::new();
    let mut result: TxProofWithMMRProof = TxProofWithMMRProof {
        message: array_bytes::bytes2hex("0x", &message),
        block_header: array_bytes::bytes2hex("0x", header.encode()),
        events_proof_str: array_bytes::bytes2hex(
            "0x",
            event_proof
                .iter()
                .map(|x| &x.0)
                .collect::<Vec<&Vec<u8>>>()
                .encode(),
        ),
        root: array_bytes::bytes2hex("0x", mmr_root),
        mmrindex: mmrblock,
        peaks: Default::default(),
        siblings: Default::default(),
        signatures: Default::default(),
        signers: Default::default(),
    };

    if let Some(header_proof) = proof {
        let proof: Option<FormatedMMR> = header_proof.into();
        if let Some(formated_proof) = proof {
            result.peaks = formated_proof.peaks;
            result.siblings = formated_proof.siblings;
        }
    }

    // confirmed block
    let events = darwinia
        .get_events_from_block_number(signblock as u32)
        .await?;
    for event in events {
        if let EventInfo::MMRRootSignedEvent(signed_info) = event {
            let mut sign_accounts = vec![];
            let mut signatures = vec![];
            for signature in signed_info.signatures {
                sign_accounts.push(signature.0.to_string());
                signatures.push(array_bytes::bytes2hex("0x", signature.1.encode()));
            }
            //result.insert("sign-account".to_string(), sign_accounts.join(","));
            result.signatures = signatures;
            result.signers = sign_accounts;
        }
    }

    Ok(TaskTerminal::new(format!("{}", result)))
}
