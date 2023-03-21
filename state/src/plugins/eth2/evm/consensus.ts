import {ethers} from "ethers";
import {JsonRpcProvider} from "ethers/src.ts/providers/provider-jsonrpc";
import BigNumber from "bignumber.js";
import {ContractRunner} from "ethers/src.ts/providers/contracts";

import consensusChainLightClientABI from '../abis/BeaconLightClient.json';

export class ConsensusLightClient {
  client: ContractRunner;
  contract: ethers.Contract;

  constructor(client: ContractRunner, address: string) {
    this.client = client;
    // @ts-ignore
    this.contract = new ethers.Contract(address, consensusChainLightClientABI, client);
  }

  async finalizedHeader() {
    return await this.contract.finalized_header();
  }

  async syncCommitteeRoots(period: BigNumber) {
    console.log('----->', period);
    return await this.contract.sync_committee_roots(period);
  }
}
