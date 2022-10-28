import {ethers} from "ethers";
import inboundABI from "@/plugins/eth2/abis/Inbound.json";
import outboundABI from "@/plugins/eth2/abis/Outbound.json";

export function MessageClient(client, options) {
  this.client = client;
  this.inbound = new ethers.Contract(options.inbound, inboundABI, client);
  this.outbound = new ethers.Contract(options.outbound, outboundABI, client);
}

const fn = MessageClient.prototype;

fn.inboundLaneNonce = async function() {
  return await this.inbound.inboundLaneNonce();
}

fn.outboundLaneNonce = async function() {
  return await this.outbound.outboundLaneNonce();
}
