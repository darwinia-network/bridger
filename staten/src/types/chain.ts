export interface ChainEndpointSubstrate {
  websocket: string;
  http?: string;
}

export interface ChainEndpointEvm {
  http: string;
}

export type ChainName = 'crab' | 'darwinia' | 'pangolin' | 'pangoro' | 'rococo' | 'moonbase' | 'ethereum';

export interface BasicChainInfo {
  name: string;
  logo: string;
  color: string;
  explorer?: string;
  currency: string;
  precision: number;
  precisionEvm?: number;
  subql?: string;
  bridge_target?: Partial<Record<ChainName, ChainBridgeTargetSolo | ChainBridgeTargetPara | ChainBridgeTargetEth>>;
}

export interface SubstrateChainInfo extends BasicChainInfo {
  endpoint: ChainEndpointSubstrate;
}

export interface EthereumChainInfo extends BasicChainInfo {
  endpoint: ChainEndpointEvm;
  consensus_chain: string;
}


export interface ChainBridgeTargetSolo {
  query_name: ChainBridgerQueryNameSolo | ChainBridgerQueryNamePara;
  lanes: string[];
}

export interface ChainBridgeTargetPara extends ChainBridgeTargetSolo {
  para_id?: number;
  relay_chain: string;
}

export interface ChainBridgeTargetEth {
  contract: ChainBridgeTargetEvmContractSubstrate | ChainBridgeTargetEvmContractEthereumExecution;
}

export interface ChainBridgeTargetEvmContractSubstrate {
  lc_consensus: string;
  lc_execution: string;
  inbound: string;
  outbound: string;
  feemarket: string;
  chain_message_committer: string;
  lane_message_committer: string;
}

export interface ChainBridgeTargetEvmContractEthereumExecution {
  inbound: string;
  outbound: string;
  feemarket: string;
  posa: string;
}

export interface ChainBridgerQueryNameSolo {
  feemarket: string;
  grandpa: string;
  messages: string;
}

export interface ChainBridgerQueryNamePara extends ChainBridgerQueryNameSolo {
  parachain: string;
}
