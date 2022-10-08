import axios from 'axios'
import {SubqlBridgeS2S} from './bridge_s2s';

/**
 *
 * @param options { host: String, graphql: String, variable: Map<String, String>}
 */
async function query(options) {
  options = options || {};
  const {host, graphql, variable} = options;
  if (!host) {
    console.error('Missing host');
    return;
  }
  if (!graphql) {
    console.error('Missing graphql');
    return;
  }
  const response = await axios.post(host, {query: graphql, variables: variable}).then(resp => resp.data);
  return response.data;
}


export default {
  install: function (Vue) {
    Vue.prototype.$subql = {
      query,
      bridge_s2s: host => new SubqlBridgeS2S(query, host),
    }
  }
}

