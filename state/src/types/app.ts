import {EthereumChainInfo, SubstrateChainInfo} from "@/types/chain";
import {ApiPromise} from "@polkadot/api";
import {Eth2Client} from "@/plugins/eth2";
import {EvmClient} from "@/plugins/eth2/evm";
import {ExecutionClient} from "@/plugins/eth2/execution";
import {ConsensusClient} from "@/plugins/eth2/consensus";


export interface AppSettings {
  allowDisabled: boolean;
  enableTestnet: boolean;
}


export interface BridgeSubstrateChainInfo extends SubstrateChainInfo {
  bridge_chain_name: string;
}

export interface BridgeEthereumChainInfo extends EthereumChainInfo {
  bridge_chain_name: string;
}

export interface SoloWithSoloChainPair {
  sourceChain: BridgeSubstrateChainInfo;
  targetChain: BridgeSubstrateChainInfo;
}

export interface SoloWithParaChainPair {
  soloChain: BridgeSubstrateChainInfo;
  paraChain: BridgeSubstrateChainInfo;
  relayChain: BridgeSubstrateChainInfo;
}

export interface ParaWithParaChainPair {
  leftParaChain: BridgeSubstrateChainInfo;
  rightParaChain: BridgeSubstrateChainInfo;
  leftRelayChain: BridgeSubstrateChainInfo;
  rightRelayChain: BridgeSubstrateChainInfo;
}

export interface SoloWithSoloClientPair {
  sourceClient: ApiPromise;
  targetClient: ApiPromise;
}

export interface SoloWithParaClientPair {
  paraClient: ApiPromise;
  relayClient: ApiPromise;
  soloClient: ApiPromise;
}

export interface ParaWithParaClientPair {
  leftParaClient: ApiPromise,
  rightParaClient: ApiPromise,
  leftRelayClient: ApiPromise,
  rightRelayClient: ApiPromise,
}

export interface SubstrateEvmWithEthereumChainPair {
  evm: BridgeEthereumChainInfo,
  execution: BridgeEthereumChainInfo
  consensus: BridgeEthereumChainInfo,
}

export interface SubstrateEvmWithEthereumClientPair {
  evm: EvmClient,
  execution: ExecutionClient,
  consensus: ConsensusClient,
}
