import {Contract, ethers} from "ethers";
import {ContractRunner} from "ethers/src.ts/providers/contracts";


import inboundABI from '../abis/Inbound.json';
import outboundABI from '../abis/Outbound.json';

export interface MessageOptions {
  inbound: string;
  outbound: string;
}

export class MessageClient {
  client: ContractRunner;
  inbound: Contract;
  outbound: Contract;


  constructor(client: ContractRunner, options: MessageOptions) {
    this.client = client;
    // @ts-ignore
    this.inbound = new ethers.Contract(options.inbound, inboundABI, client);
    // @ts-ignore
    this.outbound = new ethers.Contract(options.outbound, outboundABI, client);
  }


  async inboundLaneNonce() {
    return await this.inbound.inboundLaneNonce();
  }

  async outboundLaneNonce() {
    return await this.outbound.outboundLaneNonce();
  }
}
