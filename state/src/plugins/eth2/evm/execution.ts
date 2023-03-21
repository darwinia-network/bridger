import {ethers} from "ethers";
import {ContractRunner} from "ethers/src.ts/providers/contracts";


import executionChainLightClientABI from '../abis/ExecutionLayer.json';


export class ExecutionLightClient {
  client: ContractRunner;
  contract: ethers.Contract;

  constructor(client: ContractRunner, address: string) {
    this.client = client;
    // @ts-ignore
    this.contract = new ethers.Contract(address, executionChainLightClientABI, client);
  }

  async stateRoot() {
    return await this.contract.merkle_root();
  }
}
