import {ConsensusLightClient} from "@/plugins/eth2/evm/consensus";
import {ExecutionLightClient} from "@/plugins/eth2/evm/execution";


export function EvmClient(client) {
  this.client = client;
}

const fn = EvmClient.prototype;

fn.consensusLightClient = function (address) {
  return new ConsensusLightClient(this.client, address);
};

fn.executionLightClient = function(address) {
  return new ExecutionLightClient(this.client, address);
}




