import {ContractRunner, ethers} from "ethers";
import {EvmClient} from "@/plugins/eth2/evm";
import {ExecutionClient} from "@/plugins/eth2/execution";
import BigNumber from "bignumber.js";
import {EthClientOptions} from "@/plugins/eth2/types";
import {ConsensusClient} from "@/plugins/eth2/consensus";

function client(options: EthClientOptions) {
  return new ethers.JsonRpcProvider(options.endpoint)
}

export interface Eth2Toolkit {
  calcPeriod(slot: BigNumber): BigNumber;
}

export interface Eth2Client {
  ethers(): ContractRunner;

  toolkit: Eth2Toolkit;

  evm(options: EthClientOptions): EvmClient;

  execution(options: EthClientOptions): ExecutionClient;

  consensus(options: EthClientOptions): ConsensusClient;
}

export default {
  // @ts-ignore
  install: (app, options) => {
    const eth2 = {
      ethers: client,
      toolkit: {
        calcPeriod: (slot: BigNumber) => {
          let _slot = slot;
          if (!_slot.div) {
            _slot = new BigNumber(_slot);
          }
          return _slot.div(32).div(256);
        },
      },
      evm: (options: EthClientOptions) => new EvmClient(client(options)),
      execution: (options: EthClientOptions) => new ExecutionClient(client(options)),
      consensus: (options: EthClientOptions) => new ConsensusClient(options),
    };
    app.provide("eth2", eth2);
  },
};


