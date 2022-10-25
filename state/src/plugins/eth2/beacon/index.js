import axios from 'axios';

export function BeaconClient(options) {
  this.endpoint = options.endpoint;
}

const fn = BeaconClient.prototype;


fn.getSyncCommitteePeriodUpdate = async function(start_period, count) {
  let url = `${this.endpoint}/eth/v1/beacon/light_client/updates`;
  const resp = await axios.get(url, {
    params: {
      start_period,
      count,
    }
  });
  return resp.data;
}

