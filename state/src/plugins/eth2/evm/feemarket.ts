import {ethers} from "ethers";
import {ContractRunner} from "ethers/src.ts/providers/contracts";

import feemarketABI from '../abis/FeeMarket.json';

export class FeemarketClient {
  client: ContractRunner;
  contract: ethers.Contract;

  constructor(client: ContractRunner, address: string) {
    this.client = client;
    // @ts-ignore
    this.contract = new ethers.Contract(address, feemarketABI, client);
  }

  async assignedRelayers() {
    const assignedRelayers = [];
    const relayers = await this.contract.getTopRelayers();
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

