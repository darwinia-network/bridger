import {JsonRpcProvider} from "ethers/src.ts/providers/provider-jsonrpc";
import {ConsensusLightClient} from "@/plugins/eth2/evm/consensus";
import {ExecutionLightClient} from "@/plugins/eth2/evm/execution";
import {MessageClient, MessageOptions} from "@/plugins/eth2/common/message";
import {FeemarketClient} from "@/plugins/eth2/evm/feemarket";

export class EvmClient {
  client: JsonRpcProvider;

  constructor(client: JsonRpcProvider) {
    this.client = client;
  }

  consensusLightClient(address: string): ConsensusLightClient {
    return new ConsensusLightClient(this.client, address);
  }

  executionLightClient(address: string): ExecutionLightClient {
    return new ExecutionLightClient(this.client, address);
  }

  message(options: MessageOptions): MessageClient {
    return new MessageClient(this.client, options);
  }

  feemarket(address: string): FeemarketClient {
    return new FeemarketClient(this.client, address);
  }

}


