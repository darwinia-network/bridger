import {EthereumApi} from "@/plugins/eth2/ethereum/api";

export function EthereumClient(client) {
  this.client = client;
}

const fn = EthereumClient.prototype;

fn.api = function(host) {
  return new EthereumApi(host);
}
