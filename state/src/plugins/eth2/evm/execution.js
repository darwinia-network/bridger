import {ethers} from "ethers";


const executionChainLightClientABI = require('../abis/ExecutionLayer.json');

export function ExecutionLightClient(client, address) {
  this.client = client;
  this.contract = new ethers.Contract(address, executionChainLightClientABI, client);
}

const fn = ExecutionLightClient.prototype;

fn.stateRoot = async function(block) {
  return await this.contract.merkle_root();
}
