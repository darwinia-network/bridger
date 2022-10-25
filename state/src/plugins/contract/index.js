import { ethers } from "ethers";
import {DarwiniaClient} from "@/plugins/contract/darwinia";
import {EthereumClient} from "@/plugins/contract/ethereum";

function client(options) {
  return new ethers.providers.JsonRpcProvider(options.endpoint)
}

export default {
  install: function (Vue) {
    Vue.prototype.$contract = {
      ethers: client,
      darwinia: options => new DarwiniaClient(client(options)),
      ethereum: options => new EthereumClient(client(options)),
    }
  }
}

