import {MessageClient} from "@/plugins/eth2/common/message";

export function ExecutionClient(client) {
  this.client = client;
}

const fn = ExecutionClient.prototype;

fn.message = function (options) {
  return new MessageClient(this.client, options);
}
