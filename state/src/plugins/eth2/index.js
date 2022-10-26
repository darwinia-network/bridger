import { ethers } from "ethers";
import {EvmClient} from "@/plugins/eth2/evm";
import {ExecutionClient} from "@/plugins/eth2/execution";
import {ConsensusClient} from "@/plugins/eth2/consensus";
import BigNumber from "bignumber.js";

function client(options) {
  return new ethers.providers.JsonRpcProvider(options.endpoint)
}

export default {
  install: function (Vue) {
    Vue.prototype.$eth2 = {
      ethers: client,
      toolkit: {
        calcPeriod: slot => {
          let _slot = slot;
          if (!_slot.div) {
            _slot = new BigNumber(_slot);
          }
          return _slot.div(32).div(256);
        },
      },
      evm: options => new EvmClient(client(options)),
      execution: options => new ExecutionClient(client(options)),
      consensus: options => new ConsensusClient(options)
    }
  }
}

