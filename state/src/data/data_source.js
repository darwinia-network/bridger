import * as bridgeGroups from './bridge_groups'
import * as chainInfoData from './chain_info'
import * as graphqlData from '../plugins/subql/graphql_query'

export function bridgerGroups(
  {
    allowDisabled = false,
    enableTestnet = true,
  }
) {
  const sources = bridgeGroups.default;
  const groups = [];
  for (const group of sources) {
    const allowGroup = {name: group.name};
    const bridges = group.bridges;
    const allowBridges = [];
    for (const bridge of bridges) {
      if (!allowDisabled && !bridge.enable) {
        continue;
      }
      if (!enableTestnet && bridge.mode === 'testnet') {
        continue;
      }
      allowBridges.push(bridge);
    }
    if (allowBridges.length === 0) {
      continue;
    }
    allowGroup.bridges = allowBridges;
    groups.push(allowGroup);
  }
  return groups;
}

export function findBridge(name) {
  const sources = bridgeGroups.default;
  for (const group of sources) {
    const bridges = group.bridges;
    for (const bridge of bridges) {
      if (bridge.name === name) {
        return {group: group.name, bridge};
      }
    }
  }
}

export function chainInfo(chain) {
  return chainInfoData.default[chain];
}

