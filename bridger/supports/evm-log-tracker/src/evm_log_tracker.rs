use std::marker::PhantomData;
use std::time::Duration;

use tokio::time::sleep;

use web3::types::{Log, H160, H256};

use crate::{EvmChain, EvmClient, LogsHandler, Result};

#[derive(Debug)]
pub struct EvmLogTracker<C: EvmChain, H: LogsHandler> {
    client: EvmClient,
    topics_list: Vec<(H160, Vec<H256>)>,
    logs_handler: H,
    from: u64,
    step_in_secs: u64,
    pub running: bool,
    phantom: PhantomData<C>,
}

impl<C: EvmChain, H: LogsHandler> EvmLogTracker<C, H> {
    pub fn new(
        client: EvmClient,
        topics_list: Vec<(H160, Vec<H256>)>,
        logs_handler: H,
        from: u64,
        step_in_secs: u64,
    ) -> Self {
        EvmLogTracker {
            client,
            topics_list,
            logs_handler,
            from,
            step_in_secs,
            running: false,
            phantom: PhantomData,
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        self.running = true;
        loop {
            match self.next().await {
                Err(err) => {
                    return Err(err);
                }
                Ok(logs) => {
                    self.handle(logs).await?;
                }
            }

            if !self.running {
                break;
            }

            sleep(Duration::from_secs(self.step_in_secs)).await;
        }

        Ok(())
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub async fn next(&mut self) -> Result<Vec<Log>> {
        let mut result = vec![];
        let (from, to) = C::next_range(self.from, &self.client).await?;
        info!(
            "Heartbeat>>> Scanning on {} for new cross-chain transactions from {} to {} ...",
            C::NAME,
            from,
            to
        );
        for topics in &self.topics_list {
            let logs = self.client.get_logs(&topics.0, &topics.1, from, to).await?;
            result.extend_from_slice(&logs);
        }
        self.from = to;
        Ok(result)
    }

    async fn handle(&mut self, logs: Vec<Log>) -> Result<()> {
        self.logs_handler
            .handle(&self.client, &self.topics_list, logs)
            .await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_ethereum() {
    use crate::DefaultLogsHandler;
    use crate::Ethereum;
    use array_bytes::hex2bytes_unchecked as bytes;
    use web3::transports::Http;
    use web3::Web3;
    let web3 = Web3::new(
        Http::new("https://ropsten.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80").unwrap(),
    );

    let contract_address = "0xD35Bb6F1bc1C84b53E0995c1830454AB7C4147f1";
    let contract_address = H160::from_slice(&bytes(contract_address));

    let topics = &vec!["0x96635f5f1b0b05ed7e2265d4e13634378280f038e5a958227d4f383f825c2771"];
    let topics = topics.iter().map(|t| H256::from_slice(&bytes(t))).collect();

    let client = EvmClient::new(web3);
    let mut tracker = EvmLogTracker::<Ethereum, DefaultLogsHandler>::new(
        client,
        vec![(contract_address, topics)],
        DefaultLogsHandler {},
        100,
        1,
    );

    let res = tracker.next().await;
    assert!(res.unwrap().len() > 0);
}

#[tokio::test]
async fn test_heco() {
    use crate::DefaultLogsHandler;
    use crate::Heco;
    use array_bytes::hex2bytes_unchecked as bytes;
    use web3::transports::Http;
    use web3::Web3;
    let web3 = Web3::new(Http::new("https://http-mainnet-node.huobichain.com").unwrap());

    let contract_address = "0x0981F3C078856E2491F11A5F86d26274Bb4009d2";
    let contract_address = H160::from_slice(&bytes(contract_address));

    let topics = &vec!["0x2709918445f306d3e94d280907c62c5d2525ac3192d2e544774c7f181d65af3e"];
    let topics = topics.iter().map(|t| H256::from_slice(&bytes(t))).collect();

    let client = EvmClient::new(web3);
    let mut tracker = EvmLogTracker::<Heco, DefaultLogsHandler>::new(
        client,
        vec![(contract_address, topics)],
        DefaultLogsHandler {},
        4006177,
        1,
    );

    let res = tracker.next().await;
    assert!(res.unwrap().len() > 0);
}
