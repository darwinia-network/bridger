export default [
  {
    name: 'S2S',
    bridges: [
      {enable: true, name: 'darwinia-crab', mode: 'mainnet', bridge_type: 's2s'},
      {enable: true, name: 'crab-crabparachain', mode: 'mainnet', bridge_type: 'parachain'},
      {enable: true, name: 'pangolin-pangoro', mode: 'testnet', bridge_type: 's2s'},
      {enable: true, name: 'pangolin-pangolinparachain', mode: 'testnet', bridge_type: 'parachain'},
    ],
  },
  {
    name: 'E2E',
    bridges: [
      {enable: false, name: 'darwinia-ethereum', mode: 'mainnet'},
      {enable: false, name: 'pangoro-goerli', mode: 'testnet'},
    ],
  }
];
