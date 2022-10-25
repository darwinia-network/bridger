import {BeaconLightClient} from "@/plugins/eth2/evm/beacon";


export function EvmClient(client) {
  this.client = client;
}

const fn = EvmClient.prototype;

fn.beaconLightClient = function (address) {
  return new BeaconLightClient(this.client, address);
};




