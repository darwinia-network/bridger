use std::str::FromStr;

use web3::{contract::Contract, transports::Http, types::Address, Web3};

pub struct Outbound {
    pub contract: Contract<Http>,
}

impl Outbound {
    pub fn new(client: Web3<Http>, address: &str) -> color_eyre::Result<Self> {
        let contract = Contract::from_json(
            client.eth(),
            Address::from_str(address)?,
            include_bytes!("Outbound.json"),
        )?;
        Ok(Self { contract })
    }

    pub async fn outbound_lane_nonce(&self) -> color_eyre::Result<()> {
        todo!()
    }

    pub async fn send_message(&self) -> color_eyre::Result<()> {
        todo!()
    }

    pub async fn data(&self) -> color_eyre::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outbound() {
        let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        Outbound::new(client, "0x4214611Be6cA4E337b37e192abF076F715Af4CaE").unwrap();
    }
}
