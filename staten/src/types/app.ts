import {SubstrateChainInfo} from "@/types/chain";
import {ApiPromise} from "@polkadot/api";


export interface AppSettings {
  allowDisabled: boolean;
  enableTestnet: boolean;
}


export interface BridgeSubstrateChainInfo extends SubstrateChainInfo {
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
