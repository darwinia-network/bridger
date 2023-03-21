import axios from 'axios'
import {SubqlBridgeS2S} from "@/plugins/subql/bridge_s2s";
import {SubqlBridgeE2E} from "@/plugins/subql/bridge_e2e";


/**
 *
 * @param options { host: String, graphql: String, variable: Map<String, String>}
 */
async function query(options: {
  host: string;
  graphql: string;
  variable: Record<string, any>,
}) {
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
  const response = await axios.post(host, {query: graphql, variables: variable})
    .then(resp => resp.data);
  return response.data;
}

export interface Subql {
  query: Function,
  bridge_s2s: Function,
  bridge_e2e: Function,
}

export default {
  // @ts-ignore
  install: (app, options) => {
    const subql = {
      query,
      bridge_s2s: (host: string) => new SubqlBridgeS2S(query, host),
      bridge_e2e: (host: string) => new SubqlBridgeE2E(query, host),
    };
    app.provide("subql", subql);
  },
};
