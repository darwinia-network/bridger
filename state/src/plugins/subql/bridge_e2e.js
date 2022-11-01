import {
  BRIDGE_E2E_LAST_COLLECTED_AUTHORITIES_CHANGE_SIGNATURE_EVENT,
  BRIDGE_E2E_LAST_COLLECTED_NEW_MESSAGE_ROOT_SIGNATURES_EVENT,
  BRIDGE_E2E_LAST_COLLECTING_AUTHORITIES_CHANGE_SIGNATURE_EVENT,
  BRIDGE_E2E_LAST_COLLECTING_NEW_MESSAGE_ROOT_SIGNATURES_EVENT
} from "@/plugins/subql/graphql_query";
import is from "is_js";


export function SubqlBridgeE2E(query, host) {
  this.query = query;
  this.host = host;
}

const fn = SubqlBridgeE2E.prototype;

fn.lastCollectingMessageRootSignatureEvent = async function() {
  const ret = await this.query({
    host: this.host,
    graphql: BRIDGE_E2E_LAST_COLLECTING_NEW_MESSAGE_ROOT_SIGNATURES_EVENT,
    variable: {},
  });
  const nodes = ret['collectingNewMessageRootSignaturesEvents']['nodes'];
  if (is.not.empty(nodes)) {
    return nodes[0];
  }
  return null;
}

fn.lastCollectedMessageRootSignatureEvent = async function() {
  const ret = await this.query({
    host: this.host,
    graphql: BRIDGE_E2E_LAST_COLLECTED_NEW_MESSAGE_ROOT_SIGNATURES_EVENT,
    variable: {},
  });
  const nodes = ret['collectedEnoughNewMessageRootSignaturesEvents']['nodes'];
  if (is.not.empty(nodes)) {
    return nodes[0];
  }
  return null;
}

fn.lastCollectingAuthoritiesChangeSignatureEvent = async function() {
  const ret = await this.query({
    host: this.host,
    graphql: BRIDGE_E2E_LAST_COLLECTING_AUTHORITIES_CHANGE_SIGNATURE_EVENT,
    variable: {},
  });
  const nodes = ret['collectingAuthoritiesChangeSignaturesEvents']['nodes'];
  if (is.not.empty(nodes)) {
    return nodes[0];
  }
  return null;
}

fn.lastCollectedAuthoritiesChangeSignatureEvent = async function() {
  const ret = await this.query({
    host: this.host,
    graphql: BRIDGE_E2E_LAST_COLLECTED_AUTHORITIES_CHANGE_SIGNATURE_EVENT,
    variable: {},
  });
  const nodes = ret['collectedEnoughAuthoritiesChangeSignaturesEvents']['nodes'];
  if (is.not.empty(nodes)) {
    return nodes[0];
  }
  return null;
}
