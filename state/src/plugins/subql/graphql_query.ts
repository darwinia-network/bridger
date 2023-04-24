/**
 * next mandatory header
 * @type {string}
 */
export const BRIDGE_S2S_NEXT_MANDATORY_BLOCK = `
query queryNextRelayBlock($block: Int!) {
  needRelayBlocks (
    first: 1
    orderBy: BLOCK_NUMBER_ASC
    filter: {
      blockNumber: {
        greaterThan: $block
      }
      type: {
        equalTo: "mandatory"
      }
    }
  ) {
    nodes {
      id
      blockNumber
      blockHash
      type
      origin
      laneId
      messageNonce
      parentHash
      stateRoot
      extrinsicsRoot
      digest
      onDemandType
      additional
      timestamp
    }
  }
}
`;

/**
 * next on-demand block
 * @type {string}
 */
export const BRIDGE_S2S_NEXT_ON_DEMAND_BLOCK = `
query queryNextRelayBlock($origin: String!) {
  needRelayBlocks (
    first: 1
    orderBy: BLOCK_NUMBER_DESC
    filter: {
      type: {
        equalTo: "on-demand"
      }
      origin: {
        equalTo: $origin
      }
    }
  ) {
    nodes {
      id
      blockNumber
      blockHash
      type
      origin
      laneId
      messageNonce
      parentHash
      stateRoot
      extrinsicsRoot
      digest
      onDemandType
      additional
      timestamp
    }
  }
}
`;

/**
 * next candidate included event
 * @type {string}
 */
export const BRIDGE_S2S_NEXT_CANDIDATE_INCLUDED_EVENT = `
query queryNextCandidateIncludedEvent($para_head: String!) {
  candidateIncludedEvents (
    first: 1
    orderBy: INCLUDED_RELAY_BLOCK_ASC
    filter: {
      paraHead: {
          equalTo: $para_head
      }
    }
  ) {
    nodes {
      id
      includedRelayBlock
      paraId
      signature
      paraHead
      relayParent
    }
  }
}
`;

/**
 * last collecting new message root signatures event
 * @type {string}
 */
export const BRIDGE_E2E_LAST_COLLECTING_NEW_MESSAGE_ROOT_SIGNATURES_EVENT = `
query lastCollectingNewMessageRootSignaturesEvent {
  collectingNewMessageRootSignaturesEvents(
    orderBy: BLOCK_NUMBER_DESC
    first: 1
  ) {
    nodes {
      id
      blockNumber
      blockHash
      message
    }
  }
}
`;

/**
 * last collected new message root signatures event
 * @type {string}
 */
export const BRIDGE_E2E_LAST_COLLECTED_NEW_MESSAGE_ROOT_SIGNATURES_EVENT = `
query nextCollectedEnoughNewMessageRootSignaturesEvent {
  collectedEnoughNewMessageRootSignaturesEvents(
    orderBy: BLOCK_NUMBER_DESC
    first: 1
  ) {
    nodes {
      id
      blockNumber
      blockHash
      message
      commitmentBlockNumber
      commitmentNonce
      signatures {
        nodes {
          id
          address
          signature
        }
      }
    }
  }
}
`;

export const BRIDGE_E2E_LAST_COLLECTING_AUTHORITIES_CHANGE_SIGNATURE_EVENT = `
query nextCollectingAuthoritiesChangeSignaturesEvent {
  collectingAuthoritiesChangeSignaturesEvents(
    orderBy: BLOCK_NUMBER_DESC
    first: 1
  ) {
    nodes {
      id
      blockNumber
      blockHash
      message
    }
  }
}
`;

export const BRIDGE_E2E_LAST_COLLECTED_AUTHORITIES_CHANGE_SIGNATURE_EVENT = `
query nextCollectedEnoughAuthoritiesChangeSignaturesEven {
  collectedEnoughAuthoritiesChangeSignaturesEvents(
    orderBy: BLOCK_NUMBER_DESC
    first: 1
  ) {
    nodes {
      id
      blockHash
      blockNumber
      message
      operationType
      operationPre
      operationNew
      operationOld
      threshold
      signatures {
        nodes {
          id
          address
          signature
        }
      }
    }
  }
}
`;
