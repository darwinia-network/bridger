import {SubstrateChainInfo} from "@/types/chain";
import {ApiPromise} from "@polkadot/api";


export interface AppSettings {
  allowDisabled: boolean;
  enableTestnet: boolean;
}


export interface BridgeSubstrateChainInfo extends SubstrateChainInfo {
  bridge_chain_name: string;
}

export interface ParaWithParaChainPair {
  leftParaChain: BridgeSubstrateChainInfo;
  rightParaChain: BridgeSubstrateChainInfo;
  leftRelayChain: BridgeSubstrateChainInfo;
  rightRelayChain: BridgeSubstrateChainInfo;
}

export interface ParaWithParaClientPair {
  leftParaClient: ApiPromise,
  rightParaClient: ApiPromise,
  leftRelayClient: ApiPromise,
  rightRelayClient: ApiPromise,
}
