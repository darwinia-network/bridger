
export interface BridgeGroup {
  name: string;
  bridges: Bridge[];
}

export interface Bridge {
  name: string;
  enable: boolean;
  mode?: string;
}

