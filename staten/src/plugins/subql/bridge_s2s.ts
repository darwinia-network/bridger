

// export function SubqlBridgeS2S(query: Function , host: string) {
//   this.query = query;
//   this.host = host;
// }

import {
  BRIDGE_S2S_NEXT_CANDIDATE_INCLUDED_EVENT,
  BRIDGE_S2S_NEXT_MANDATORY_BLOCK,
  BRIDGE_S2S_NEXT_ON_DEMAND_BLOCK
} from "@/plugins/subql/graphql_query";

export class SubqlBridgeS2S {
  query: Function;
  host: string;

  constructor(query: Function, host: string) {
    this.query = query;
    this.host = host;
  }


  async nextMandatoryBlock(block: string | number) {
    const ret = await this.query({
      host: this.host,
      graphql: BRIDGE_S2S_NEXT_MANDATORY_BLOCK,
      variable: {block},
    });
    const nodes = ret['needRelayBlocks']['nodes'];
    if (nodes && nodes.length) {
      return nodes[0];
    }
    return null;
  }


  async nextOnDemandBlock(origin: string) {
    if (origin.indexOf('parachain') > -1) {
      // the subql stored bridge name such as bridge-pangolin-parachain
      // but the real/binary name is bridge-pangolinparachain.
      // so there need replace parachain to -parachain
      if (origin.indexOf('-parachain') === -1) {
        origin = origin.replace('parachain', '-parachain');
      }
    }
    const ret = await this.query({
      host: this.host,
      graphql: BRIDGE_S2S_NEXT_ON_DEMAND_BLOCK,
      variable: {origin},
    });
    const nodes = ret['needRelayBlocks']['nodes'];
    if (nodes && nodes.length) {
      return nodes[0];
    }
    return null;
  }

  async queryNextCandidateIncludedEvent(paraHead: string) {
    const ret = await this.query({
      host: this.host,
      graphql: BRIDGE_S2S_NEXT_CANDIDATE_INCLUDED_EVENT,
      variable: {para_head: paraHead}
    });
    const nodes = ret['candidateIncludedEvents']['nodes'];
    if (nodes && nodes.length) {
      return nodes[0];
    }
    return null;
  }

}

