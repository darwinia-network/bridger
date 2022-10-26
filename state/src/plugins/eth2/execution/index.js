import {MessageClient} from "@/plugins/eth2/common/message";
import {PosaLightClient} from "@/plugins/eth2/execution/posa";

export function ExecutionClient(client) {
  this.client = client;
}

const fn = ExecutionClient.prototype;

fn.message = function (options) {
  return new MessageClient(this.client, options);
}

fn.posaLightClient = function(address) {
  return new PosaLightClient(this.client, address);
}
