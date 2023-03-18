import {MessageClient, MessageOptions} from "@/plugins/eth2/common/message";
import {PosaLightClient} from "@/plugins/eth2/execution/posa";
import {FeemarketClient} from "@/plugins/eth2/execution/feemarket";
import {ContractRunner} from "ethers";

// export function ExecutionClient(client) {
//   this.client = client;
// }
//
// const fn = ExecutionClient.prototype;
//
// fn.message = function (options) {
//   return new MessageClient(this.client, options);
// }
//
// fn.posaLightClient = function(address) {
//   return new PosaLightClient(this.client, address);
// }
//
// fn.feemarket = function(address) {
//   return new FeemarketClient(this.client, address);
// }

export class ExecutionClient {
  client: ContractRunner;


  constructor(client: ContractRunner) {
    this.client = client;
  }

  message(options: MessageOptions): MessageClient {
    return new MessageClient(this.client, options);
  }

  posaLightClient(address: string): PosaLightClient {
    return new PosaLightClient(this.client, address);
  }

  feemarket(address: string): FeemarketClient {
    return new FeemarketClient(this.client, address);
  }
}
