import {
  BRIDGE_E2E_LAST_COLLECTED_AUTHORITIES_CHANGE_SIGNATURE_EVENT,
  BRIDGE_E2E_LAST_COLLECTED_NEW_MESSAGE_ROOT_SIGNATURES_EVENT,
  BRIDGE_E2E_LAST_COLLECTING_AUTHORITIES_CHANGE_SIGNATURE_EVENT,
  BRIDGE_E2E_LAST_COLLECTING_NEW_MESSAGE_ROOT_SIGNATURES_EVENT
} from "@/plugins/subql/graphql_query";


export class SubqlBridgeE2E {
  query: Function;
  host: string;

  constructor(query: Function, host: string) {
    this.query = query;
    this.host = host;
  }


  async lastCollectingMessageRootSignatureEvent() {
    const ret = await this.query({
      host: this.host,
      graphql: BRIDGE_E2E_LAST_COLLECTING_NEW_MESSAGE_ROOT_SIGNATURES_EVENT,
      variable: {},
    });
    const nodes = ret['collectingNewMessageRootSignaturesEvents']['nodes'];
    if (nodes && nodes.length) {
      return nodes[0];
    }
    return null;
  }

  async lastCollectedMessageRootSignatureEvent() {
    const ret = await this.query({
      host: this.host,
      graphql: BRIDGE_E2E_LAST_COLLECTED_NEW_MESSAGE_ROOT_SIGNATURES_EVENT,
      variable: {},
    });
    const nodes = ret['collectedEnoughNewMessageRootSignaturesEvents']['nodes'];
    if (nodes && nodes.length) {
      return nodes[0];
    }
    return null;
  }

  async lastCollectingAuthoritiesChangeSignatureEvent() {
    const ret = await this.query({
      host: this.host,
      graphql: BRIDGE_E2E_LAST_COLLECTING_AUTHORITIES_CHANGE_SIGNATURE_EVENT,
      variable: {},
    });
    const nodes = ret['collectingAuthoritiesChangeSignaturesEvents']['nodes'];
    if (nodes && nodes.length) {
      return nodes[0];
    }
    return null;
  }

  async lastCollectedAuthoritiesChangeSignatureEvent() {
    const ret = await this.query({
      host: this.host,
      graphql: BRIDGE_E2E_LAST_COLLECTED_AUTHORITIES_CHANGE_SIGNATURE_EVENT,
      variable: {},
    });
    const nodes = ret['collectedEnoughAuthoritiesChangeSignaturesEvents']['nodes'];
    if (nodes && nodes.length) {
      return nodes[0];
    }
    return null;
  }
}
