import {
  BRIDGE_S2S_NEXT_CANDIDATE_INCLUDED_EVENT,
  BRIDGE_S2S_NEXT_MANDATORY_BLOCK,
  BRIDGE_S2S_NEXT_ON_DEMAND_BLOCK
} from './graphql_query'
import is from 'is_js';

export function SubqlBridgeS2S(query, host) {
  this.query = query;
  this.host = host;
}

const fn = SubqlBridgeS2S.prototype;

/**
 * next mandatory block
 * @param block number
 * @returns {Promise<null|*>}
 */
fn.nextMandatoryBlock = async function (block) {
  const ret = await this.query({
    host: this.host,
    graphql: BRIDGE_S2S_NEXT_MANDATORY_BLOCK,
    variable: {block},
  });
  const nodes = ret['needRelayBlocks']['nodes'];
  if (is.not.empty(nodes)) {
    return nodes[0];
  }
  return null;
}

/**
 * next on-demand block
 * @param origin bridge origin
 * @returns {Promise<void>}
 */
fn.nextOnDemandBlock = async function (origin) {
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
  if (is.not.empty(nodes)) {
    return nodes[0];
  }
  return null;
}

/**
 * query next candidate include event
 * @param paraHead
 * @returns {Promise<void>}
 */
fn.queryNextCandidateIncludedEvent = async function(paraHead) {
  const ret = await this.query({
    host: this.host,
    graphql: BRIDGE_S2S_NEXT_CANDIDATE_INCLUDED_EVENT,
    variable: {para_head: paraHead}
  });
  const nodes = ret['candidateIncludedEvents']['nodes'];
  if (is.not.empty(nodes)) {
    return nodes[0];
  }
  return null;
}
