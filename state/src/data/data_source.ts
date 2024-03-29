import {Bridge, BridgeGroup, BridgeInfo} from "@/types/bridge";
import * as bridgeGroups from './bridge_groups'
import * as chainInfoData from './chain_info'
import {AppSettings} from "@/types/app";
import {BasicChainInfo, EthereumChainInfo, SubstrateChainInfo} from "@/types/chain";


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

export function findBridge(name: string): BridgeInfo | undefined {
  console.log(name);
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

export function chainInfo(chain: string): BasicChainInfo | SubstrateChainInfo | EthereumChainInfo {
  return chainInfoData.default[chain];
}
