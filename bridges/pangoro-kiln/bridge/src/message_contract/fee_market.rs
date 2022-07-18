use std::str::FromStr;

use web3::{contract::Contract, transports::Http, types::Address, Web3};

pub struct FeeMarket {
    pub contract: Contract<Http>,
}

impl FeeMarket {
    pub fn new(client: Web3<Http>, address: &str) -> color_eyre::Result<Self> {
        let contract = Contract::from_json(
            client.eth(),
            Address::from_str(address)?,
            include_bytes!("FeeMarket.json"),
        )?;
        Ok(Self { contract })
    }

    pub async fn enroll(&self) -> color_eyre::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fee_market() {
        let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        FeeMarket::new(client, "0x9FbA8f0a0Bd6CbcB6283c042edc6b20894Be09c8").unwrap();
    }
}
