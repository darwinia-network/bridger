import {BridgeGroup} from "@/types/bridge";

export default [
  {
    name: 'S2S',
    bridges: [
      {enable: true, name: 'darwinia-crab', mode: 'mainnet', bridge_type: 'solo-with-solo'},
      // {enable: true, name: 'crab-crabparachain', mode: 'mainnet', bridge_type: 'solo-with-para'},
      {enable: true, name: 'pangolin-pangoro', mode: 'testnet', bridge_type: 'para-with-para'},
    ],
  },
  {
    name: 'E2E',
    bridges: [
      {enable: true, name: 'darwinia-ethereum', mode: 'mainnet'},
      {enable: false, name: 'pangoro-goerli', mode: 'testnet'},
    ],
  }
] as BridgeGroup[];
