import {MessageClient} from "@/plugins/eth2/common/message";
import {PosaLightClient} from "@/plugins/eth2/execution/posa";
import {FeemarketClient} from "@/plugins/eth2/execution/feemarket";

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

fn.feemarket = function(address) {
  return new FeemarketClient(this.client, address);
}
