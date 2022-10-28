import { ethers } from "ethers";

const consensusChainLightClientABI = require('../abis/BeaconLightClient.json');

export function ConsensusLightClient(client, address) {
  this.client = client;
  this.contract = new ethers.Contract(address, consensusChainLightClientABI, client);
}

const fn = ConsensusLightClient.prototype;

/**
 *
 * @returns { slot, proposer_index, parent_root, state_root, body_root }
 */
fn.finalizedHeader = async function () {
  return await this.contract.finalized_header();
}

/**
 *
 * @returns {Promise<void>}
 */
fn.syncCommitteeRoots = async function(period) {
  return await this.contract.sync_committee_roots(period);
}
