import {ethers} from "ethers";
import feemarketABI from "@/plugins/eth2/abis/FeeMarket.json";


export function FeemarketClient(client, address) {
  this.client = client;
  this.contract = new ethers.Contract(address, feemarketABI, client);
}

const fn = FeemarketClient.prototype;

fn.assignedRelayers = async function() {
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


