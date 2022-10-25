import { ethers } from "ethers";

const beaconChainLightClientABI = require('../abis/BeaconLightClient.json');

export function BeaconLightClient(client, address) {
  this.client = client;
  // this.contract = new web3.eth.Contract(
  //   beaconChainLightClient,
  //   address,
  // );
  this.contract = new ethers.Contract(address, beaconChainLightClientABI, client);
}

const fn = BeaconLightClient.prototype;

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
