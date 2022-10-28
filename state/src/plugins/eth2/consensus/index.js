import axios from 'axios';

export function ConsensusClient(options) {
  this.endpoint = options.endpoint;
}

const fn = ConsensusClient.prototype;


fn.syncCommitteePeriodUpdate = async function(start_period, count) {
  const url = `${this.endpoint}/eth/v1/beacon/light_client/updates`;
  const resp = await axios.get(url, {
    params: {
      start_period,
      count,
    }
  });
  return resp.data;
}

fn.finalityUpdate = async function() {
  const url = `${this.endpoint}/eth/v1/beacon/light_client/finality_update/`;
  const resp = await axios.get(url);
  return resp.data;
}

fn.header = async function(id) {
  const url = `${this.endpoint}/eth/v1/beacon/headers/${id}`;
  const resp = await axios.get(url);
  return resp.data;
}

fn.block = async function(id) {
  const url = `${this.endpoint}/eth/v2/beacon/blocks/${id}`;
  const resp = await axios.get(url);
  return resp.data;
}

