import {ConsensusLightClient} from "@/plugins/eth2/evm/consensus";
import {ExecutionLightClient} from "@/plugins/eth2/evm/execution";
import {MessageClient} from "@/plugins/eth2/common/message";
import {FeemarketClient} from "@/plugins/eth2/evm/feemarket";


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

fn.message = function(options) {
  return new MessageClient(this.client, options);
}

fn.feemarket = function(address) {
  return new FeemarketClient(this.client, address);
}




