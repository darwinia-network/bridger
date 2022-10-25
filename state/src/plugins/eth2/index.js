import { ethers } from "ethers";
import {EvmClient} from "@/plugins/eth2/evm";
import {EthereumClient} from "@/plugins/eth2/ethereum";
import {BeaconClient} from "@/plugins/eth2/beacon";

function client(options) {
  return new ethers.providers.JsonRpcProvider(options.endpoint)
}

export default {
  install: function (Vue) {
    Vue.prototype.$eth2 = {
      ethers: client,
      evm: options => new EvmClient(client(options)),
      ethereum: options => new EthereumClient(client(options)),
      beacon: options => new BeaconClient(options)
    }
  }
}

