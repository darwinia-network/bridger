import {Contract, ContractRunner, ethers} from "ethers";

// const feemarketABI = require('');
import feemarketABI from '../abis/SimpleFeeMarket.json';


export class FeemarketClient {
  client: ContractRunner;
  contract: Contract;


  constructor(client: ContractRunner, address: string) {
    this.client = client;
    this.contract = new ethers.Contract(address, feemarketABI, client);
  }

  async assignedRelayers() {
    const assignedRelayers = [];
    const relayers = [
      await this.contract.getTopRelayer()
    ];
    for (const relayer of relayers) {
      const feeOf = await this.contract.feeOf(relayer);
      const lockedOf = await this.contract.lockedOf(relayer);
      const balanceOf = await this.contract.balanceOf(relayer);
      assignedRelayers.push({
        id: relayer,
        locked: lockedOf,
        fee: feeOf,
        balance: balanceOf,
      });
    }
    return assignedRelayers;
  }
}


