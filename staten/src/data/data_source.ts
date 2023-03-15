import {Bridge, BridgeGroup} from "@/types/bridge";
import * as bridgeGroups from './bridge_groups'
import {AppSettings} from "@/types/app";


function savedSettings(def: AppSettings): AppSettings {
  const saved = localStorage.getItem('APP_SETTINGS');
  return saved ? JSON.parse(saved) : def;
}

export function bridgerGroups(
  options?: AppSettings
): BridgeGroup[] {
  const settings = savedSettings(options ?? {
    allowDisabled: false,
    enableTestnet: true,
  });
  const sources = bridgeGroups.default;
  const groups = [];
  for (const group of sources) {
    const allowGroup: BridgeGroup = {name: group.name, bridges: []};
    const bridges = group.bridges;
    const allowBridges = [] as Bridge[];
    for (const bridge of bridges) {
      if (!bridge.enable) continue;
      if (!settings.allowDisabled && !bridge.enable) {
        continue;
      }
      if (!settings.enableTestnet && bridge.mode === 'testnet') {
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
