use bp_darwinia_core::AccountId;
use bp_darwinia_core::AccountIdConverter;
use feemarket_s2s::relay::BasicRelayStrategy;
use futures::{FutureExt, TryFutureExt};
use lifeline::{Lifeline, Service, Task};
use relay_substrate_client::{AccountIdOf, Chain, Client, TransactionSignScheme};
use relay_utils::metrics::MetricsParams;
use sp_core::Pair;
use substrate_relay_helper::messages_lane::MessagesRelayParams;
use substrate_relay_helper::TransactionParams;

use relay_pangolin_client::PangolinChain;
use relay_pangolin_parachain_client::PangolinParachainChain;
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::BridgeTask;
use crate::bridge::{BridgeBus, BridgeConfig};
use crate::bridge::{ChainInfoConfig, RelayConfig};
use crate::chains::pangolin::PangolinMessagesToPangolinParachain;
use crate::chains::pangolin_parachain::PangolinParachainMessagesToPangolin;
use crate::feemarket::{PangolinFeemarketApi, PangolinParachainFeemarketApi};
use crate::types::{MessagesPalletOwnerSigningParams, RelayHeadersAndMessagesInfo};

// /// Maximal allowed conversion rate error ratio (abs(real - stored) / stored) that we allow.
// ///
// /// If it is zero, then transaction will be submitted every time we see difference between
// /// stored and real conversion rates. If it is large enough (e.g. > than 10 percents, which is 0.1),
// /// then rational relayers may stop relaying messages because they were submitted using
// /// lesser conversion rate.
// const CONVERSION_RATE_ALLOWED_DIFFERENCE_RATIO: f64 = 0.05;

#[derive(Debug)]
pub struct MessageRelayService {
    _greet: Lifeline,
}

impl BridgeService for MessageRelayService {}

impl Service for MessageRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(&format!("{}-relay", BridgeTask::name()), async move {
            if let Err(e) = start() {
                tracing::error!(target: "pangolin-crabparachain", "{:?}", e);
                return Err(
                    BridgerError::Custom("Failed to start relay service".to_string()).into(),
                );
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

fn start() -> color_eyre::Result<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangolinParachain)?;
    let config_pangolin: ChainInfoConfig = bridge_config.pangolin;
    let config_pangolin_parachain: ChainInfoConfig = bridge_config.pangolin_parachain;
    let config_relay: RelayConfig = bridge_config.relay;

    let (source_chain, target_chain) = (
        config_pangolin.to_chain_info_with_expect_signer(config_relay.signer_pangolin.clone())?,
        config_pangolin_parachain
            .to_chain_info_with_expect_signer(config_relay.signer_pangolin_parachain.clone())?,
    );

    let relay_info = RelayHeadersAndMessagesInfo {
        source: source_chain,
        target: target_chain,
        lanes: config_relay.lanes.clone(),
        prometheus_params: config_relay.prometheus_params.clone(),
        create_relayers_fund_accounts: config_relay.create_relayers_fund_accounts,
        only_mandatory_headers: config_relay.only_mandatory_headers,
        pangolin_messages_pallet_owner_signing: MessagesPalletOwnerSigningParams {
            messages_pallet_owner: config_relay.pangolin_messages_pallet_owner.clone(),
            messages_pallet_owner_password: config_relay
                .pangolin_messages_pallet_owner_password
                .clone(),
        },
        pangolin_parachain_messages_pallet_owner_signing: MessagesPalletOwnerSigningParams {
            messages_pallet_owner: config_relay
                .pangolin_parachain_messages_pallet_owner
                .clone(),
            messages_pallet_owner_password: config_relay
                .pangolin_parachain_messages_pallet_owner_password,
        },
    };

    std::thread::spawn(move || futures::executor::block_on(bridge_relay(relay_info)))
        .join()
        .map_err(|_| BridgerError::Custom("Failed to join thread handle".to_string()))??;

    // bridge_relay(relay_info).await?;
    Ok(())
}

async fn bridge_relay(relay_info: RelayHeadersAndMessagesInfo) -> color_eyre::Result<()> {
    let pangolin_chain = relay_info.source;
    let pangolin_parachain_chain = relay_info.target;

    let pangolin_client = pangolin_chain
        .to_substrate_relay_chain::<PangolinChain>()
        .await?;
    let pangolin_parachain_client = pangolin_parachain_chain
        .to_substrate_relay_chain::<PangolinParachainChain>()
        .await?;

    let pangolin_sign = pangolin_chain.to_keypair::<PangolinChain>()?;
    let pangolin_parachain_sign =
        pangolin_parachain_chain.to_keypair::<PangolinParachainChain>()?;
    let pangolin_transactions_mortality = pangolin_chain.transactions_mortality()?;
    let pangolin_parachain_transactions_mortality =
        pangolin_parachain_chain.transactions_mortality()?;

    let lanes = relay_info.lanes;

    let metrics_params: MetricsParams = relay_info.prometheus_params.clone().into();
    let metrics_params = relay_utils::relay_metrics(metrics_params).into_params();

    // const METRIC_IS_SOME_PROOF: &str = "it is `None` when metric has been already registered; \
    // 			this is the command entrypoint, so nothing has been registered yet; \
    // 			qed";

    if relay_info.create_relayers_fund_accounts {
        let relayer_fund_acount_id = pallet_bridge_messages::relayer_fund_account_id::<
            AccountIdOf<PangolinChain>,
            AccountIdConverter,
        >();
        let relayers_fund_account_balance = pangolin_client
            .free_native_balance(relayer_fund_acount_id.clone())
            .await;
        if let Err(relay_substrate_client::Error::AccountDoesNotExist) =
            relayers_fund_account_balance
        {
            tracing::info!(target: "bridge", "Going to create relayers fund account at {}.", PangolinChain::NAME);
            create_pangolin_account(
                pangolin_client.clone(),
                pangolin_sign.clone(),
                relayer_fund_acount_id,
            )
            .await?;
        }

        let relayer_fund_acount_id = pallet_bridge_messages::relayer_fund_account_id::<
            AccountIdOf<PangolinParachainChain>,
            AccountIdConverter,
        >();
        let relayers_fund_account_balance = pangolin_parachain_client
            .free_native_balance(relayer_fund_acount_id.clone())
            .await;
        if let Err(relay_substrate_client::Error::AccountDoesNotExist) =
            relayers_fund_account_balance
        {
            tracing::info!(target: "bridge", "Going to create relayers fund account at {}.", PangolinParachainChain::NAME);
            create_pangolin_parachain_account(
                pangolin_parachain_client.clone(),
                pangolin_parachain_sign.clone(),
                relayer_fund_acount_id,
            )
            .await?;
        }
    }

    // Need 2x capacity since we consider both directions for each lane
    let mut message_relays = Vec::with_capacity(lanes.len() * 2);
    for lane in lanes {
        let lane = lane.into();
        let pangolin_feemarket_api =
            PangolinFeemarketApi::new(pangolin_client.clone(), lane, pangolin_sign.clone());
        let pangolin_parachain_feemarket_api = PangolinParachainFeemarketApi::new(
            pangolin_parachain_client.clone(),
            lane,
            pangolin_parachain_sign.clone(),
        );

        let pangolin_to_pangolin_parachain_messages = substrate_relay_helper::messages_lane::run::<
            PangolinMessagesToPangolinParachain,
        >(MessagesRelayParams {
            source_client: pangolin_client.clone(),
            source_transaction_params: TransactionParams {
                signer: pangolin_sign.clone(),
                mortality: pangolin_transactions_mortality,
            },
            target_client: pangolin_parachain_client.clone(),
            target_transaction_params: TransactionParams {
                signer: pangolin_parachain_sign.clone(),
                mortality: pangolin_parachain_transactions_mortality,
            },
            source_to_target_headers_relay: None,
            target_to_source_headers_relay: None,
            lane_id: lane,
            metrics_params: metrics_params.clone().disable(),
            standalone_metrics: None,
            relay_strategy: BasicRelayStrategy::new(
                pangolin_feemarket_api,
                AccountId::from(pangolin_sign.public().0),
            ),
        })
        .map_err(|e| format!("{}", e))
        .boxed();

        let pangolin_parachain_to_pangolin_messages = substrate_relay_helper::messages_lane::run::<
            PangolinParachainMessagesToPangolin,
        >(MessagesRelayParams {
            source_client: pangolin_parachain_client.clone(),
            source_transaction_params: TransactionParams {
                signer: pangolin_parachain_sign.clone(),
                mortality: pangolin_parachain_transactions_mortality,
            },
            target_client: pangolin_client.clone(),
            target_transaction_params: TransactionParams {
                signer: pangolin_sign.clone(),
                mortality: pangolin_transactions_mortality,
            },
            source_to_target_headers_relay: None,
            target_to_source_headers_relay: None,
            lane_id: lane,
            metrics_params: metrics_params.clone().disable(),
            standalone_metrics: None,
            relay_strategy: BasicRelayStrategy::new(
                pangolin_parachain_feemarket_api,
                AccountId::from(pangolin_parachain_sign.public().0),
            ),
        })
        .map_err(|e| format!("{}", e))
        .boxed();

        message_relays.push(pangolin_to_pangolin_parachain_messages);
        message_relays.push(pangolin_parachain_to_pangolin_messages);
    }

    relay_utils::relay_metrics(metrics_params)
        .expose()
        .await
        .map_err(|e| BridgerError::Custom(format!("{:?}", e)))?;

    if let Err(e) = futures::future::select_all(message_relays).await.0 {
        tracing::error!(target: "pangolin-crabparachain", "{:?}", e);
        return Err(BridgerError::Custom("Failed to start relay".to_string()).into());
    }
    Ok(())
}

async fn create_pangolin_account(
    _left_client: Client<PangolinChain>,
    _left_sign: <PangolinChain as TransactionSignScheme>::AccountKeyPair,
    _account_id: AccountIdOf<PangolinChain>,
) -> color_eyre::Result<()> {
    Err(BridgerError::Custom("Account creation is not supported by this bridge".to_string()).into())
}

async fn create_pangolin_parachain_account(
    _left_client: Client<PangolinParachainChain>,
    _left_sign: <PangolinParachainChain as TransactionSignScheme>::AccountKeyPair,
    _account_id: AccountIdOf<PangolinParachainChain>,
) -> color_eyre::Result<()> {
    Err(BridgerError::Custom("Account creation is not supported by this bridge".to_string()).into())
}
