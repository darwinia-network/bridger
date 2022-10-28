import {ethers} from "ethers";
import posaLightClientABI from "@/plugins/eth2/abis/POSALightClient.json";

export function PosaLightClient(client, address) {
  this.client = client;
  this.contract = new ethers.Contract(address, posaLightClientABI, client);
}

const fn = PosaLightClient.prototype;


fn.blockNumber = async function() {
  return await this.contract.block_number();
}
