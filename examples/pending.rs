use darwinia_bridger::Config;
use primitives::{frame::ethereum::game::PendingHeadersStoreExt, runtime::DarwiniaRuntime};
use sp_keyring::{sr25519::sr25519::Pair, AccountKeyring};
use substrate_subxt::{balances::*, sp_core::Pair as PairTrait, ClientBuilder, PairSigner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(None).unwrap();
    env_logger::init();

    let pair = Pair::from_string(&config.seed, None).unwrap();
    let signer = PairSigner::new(pair);
    let dest = AccountKeyring::Bob.to_account_id();
    let client = ClientBuilder::<DarwiniaRuntime>::new()
        .set_url(config.node)
        .build()
        .await?;

    let headers = client.pending_headers(None).await.unwrap();
    println!("{:?}", headers);
    // if headers.len() > 0 {
    //     for h in headers {
    //         println!("{:?}", h[0].0);
    //         println!("{:?}", h[0].1);
    //         println!("{:?}", h[0].2);
    //     }
    // }
    let hash = client.transfer(&signer, &dest, 10_000).await?;
    println!("Balance transfer extrinsic submitted: {:?}", hash);
    Ok(())
}
