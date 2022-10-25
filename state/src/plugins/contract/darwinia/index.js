import {BeaconLightClient} from "@/plugins/contract/darwinia/beacon";


export function DarwiniaClient(client) {
  this.client = client;
}

const fn = DarwiniaClient.prototype;

fn.beaconLightClient = function (address) {
  return new BeaconLightClient(this.client, address);
};




