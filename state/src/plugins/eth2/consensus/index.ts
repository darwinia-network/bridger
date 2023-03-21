import axios from 'axios';
import {EthClientOptions} from "@/plugins/eth2/types";
import BigNumber from "bignumber.js";

export class ConsensusClient {
  options: EthClientOptions;

  constructor(options: EthClientOptions) {
    this.options = options;
  }

  async syncCommitteePeriodUpdate(start_period: BigNumber, count: number) {
    const url = `${this.options.endpoint}/eth/v1/beacon/light_client/updates`;
    const resp = await axios.get(url, {
      params: {
        start_period,
        count,
      }
    });
    return resp.data;
  }

  async finalityUpdate() {
    const url = `${this.options.endpoint}/eth/v1/beacon/light_client/finality_update/`;
    const resp = await axios.get(url);
    return resp.data;
  }

  async header(id: string | number) {
    const url = `${this.options.endpoint}/eth/v1/beacon/headers/${id}`;
    const resp = await axios.get(url);
    return resp.data;
  }

  async block(id: string | number) {
    const url = `${this.options.endpoint}/eth/v2/beacon/blocks/${id}`;
    const resp = await axios.get(url);
    return resp.data;
  }
}

