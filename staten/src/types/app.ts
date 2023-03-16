import {SubstrateChainInfo} from "@/types/chain";
import {ApiPromise} from "@polkadot/api";


export interface AppSettings {
  allowDisabled: boolean;
  enableTestnet: boolean;
}


export interface ParaWithParaChainPair {
  leftParaChain: SubstrateChainInfo;
  rightParaChain: SubstrateChainInfo;
  leftRelayChain: SubstrateChainInfo;
  rightRelayChain: SubstrateChainInfo;
}

export interface ParaWithParaClientPair {
  leftParaClient: ApiPromise,
  rightParaClient: ApiPromise,
  leftRelayClient: ApiPromise,
  rightRelayClient: ApiPromise,
}
