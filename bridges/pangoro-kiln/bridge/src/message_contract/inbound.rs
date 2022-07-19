use std::str::FromStr;

use web3::{contract::Contract, transports::Http, types::Address, Web3};

pub struct Inbound {
    pub contract: Contract<Http>,
}

impl Inbound {
    pub fn new(client: Web3<Http>, address: &str) -> color_eyre::Result<Self> {
        let contract = Contract::from_json(
            client.eth(),
            Address::from_str(address)?,
            include_bytes!("Inbound.json"),
        )?;
        Ok(Self { contract })
    }

    pub async fn inbound_lane_nonce(&self) -> color_eyre::Result<()> {
        todo!()
    }

    pub async fn data(&self) -> color_eyre::Result<()> {
        todo!()
    }

    pub async fn receive_messages_proof(&self) -> color_eyre::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inbound() {
        let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        Inbound::new(client, "0xE04c799682F9509CF3D23A15F4A8ddc32648EDd4").unwrap();
    }
}
