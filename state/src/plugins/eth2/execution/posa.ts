import {Contract, ContractRunner, ethers} from "ethers";
import posaLightClientABI from '../abis/POSALightClient.json'



export class PosaLightClient {
  client: ContractRunner;
  contract: Contract;


  constructor(client: ContractRunner, address: string) {
    this.client = client;
    this.contract = new ethers.Contract(address, posaLightClientABI, client);
  }

  async blockNumber() {
    return await this.contract.block_number();
  }
}


