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
