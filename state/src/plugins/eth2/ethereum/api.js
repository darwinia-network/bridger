import axios from 'axios'

export function EthereumApi(host) {
  this.host = host;
}

const fn = EthereumApi.prototype;
