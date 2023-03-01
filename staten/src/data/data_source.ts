import {Bridge, BridgeGroup} from "@/types/bridge";
import * as bridgeGroups from './bridge_groups'


export function bridgerGroups(
  {
    allowDisabled = false,
    enableTestnet = true,
  }
): BridgeGroup[] {
  const sources = bridgeGroups.default;
  const groups = [];
  for (const group of sources) {
    const allowGroup: BridgeGroup = {name: group.name, bridges: []};
    const bridges = group.bridges;
    const allowBridges = [] as Bridge[];
    for (const bridge of bridges) {
      if (!bridge.enable) continue;
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
    allowGroup.bridges.push(...allowBridges);
    groups.push(allowGroup);
  }
  return groups;
}
