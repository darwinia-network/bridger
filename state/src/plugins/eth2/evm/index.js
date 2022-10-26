import {ConsensusLightClient} from "@/plugins/eth2/evm/consensus";


export function EvmClient(client) {
  this.client = client;
}

const fn = EvmClient.prototype;

fn.consensusLightClient = function (address) {
  return new ConsensusLightClient(this.client, address);
};




