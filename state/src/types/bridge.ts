
export interface BridgeGroup {
  name: string;
  bridges: Bridge[];
}

export interface Bridge {
  name: string;
  enable: boolean;
  mode?: string;
  bridge_type?: string;
}

export interface BridgeInfo {
  group: string;
  bridge: Bridge;
}

