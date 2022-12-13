export default [
  {
    name: 'S2S',
    bridges: [
      {enable: true, name: 'darwinia-crab', mode: 'mainnet', bridge_type: 's2s'},
      // {enable: true, name: 'crab-crabparachain', mode: 'mainnet', bridge_type: 'parachain'},
      // {enable: true, name: 'pangolin-pangoro', mode: 'testnet', bridge_type: 's2s'},
    ],
  },
  {
    name: 'E2E',
    bridges: [
      {enable: true, name: 'darwinia-ethereum', mode: 'mainnet'},
      {enable: false, name: 'pangoro-goerli', mode: 'testnet'},
    ],
  }
];
