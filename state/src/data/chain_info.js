const darwinia = {
  endpoint: {
    websocket: 'wss://rpc.darwinia.network',
  },
  name: 'Darwinia',
  logo: 'https://polkadot.js.org/apps/static/darwinia.70c5ca41..svg',
  color: 'pink',
  explorer: 'https://darwinia.subscan.io',
  currency: 'RING',
  bridge_target: {
    crab: {
      query_name: {
        feemarket: 'feeMarket',
        grandpa: 'bridgeCrabGrandpa',
        messages: 'bridgeCrabMessages',
      },
      lanes: ['0x00000000'],
    },
  },
  subql: 'https://subql.darwinia.network/subql-bridger-darwinia',
};

const crab = {
  endpoint: {
    websocket: 'wss://crab-rpc.darwinia.network',
  },
  name: 'Crab',
  logo: 'https://polkadot.js.org/apps/static/crab.ebc98461..svg',
  color: 'deep-purple',
  explorer: 'https://crab.subscan.io',
  currency: 'CRAB',
  bridge_target: {
    darwinia: {
      query_name: {
        feemarket: 'darwiniaFeeMarket',
        grandpa: 'bridgeDarwiniaGrandpa',
        messages: 'bridgeDarwiniaMessages',
      },
      lanes: ['0x00000000'],
    },
    crabparachain: {
      query_name: {
        feemarket: 'crabParachainFeeMarket',
        grandpa: 'bridgeKusamaGrandpa',
        messages: 'bridgeCrabParachainMessages',
        parachains: 'bridgeKusamaParachain',
      },
      lanes: ['0x70616372'],
    },
  },
  subql: 'https://subql.darwinia.network/subql-bridger-crab',
};

const pangolin = {
  endpoint: {
    websocket: 'wss://pangolin-rpc.darwinia.network',
  },
  name: 'Pangolin',
  logo: 'https://polkadot.js.org/apps/static/pangolin.db0518f6..svg',
  color: 'teal',
  explorer: 'https://pangolin.subscan.io',
  currency: 'PRING',
  bridge_target: {
    pangoro: {
      query_name: {
        feemarket: 'pangoroFeeMarket',
        grandpa: 'bridgePangoroGrandpa',
        messages: 'bridgePangoroMessages',
      },
      lanes: ['0x726f6c69'],
    },
    pangolinparachain: {
      query_name: {
        feemarket: 'pangolinParachainFeeMarket',
        grandpa: 'bridgeRococoGrandpa',
        messages: 'bridgePangolinParachainMessages',
        parachains: 'bridgeRococoParachain',
      },
      lanes: ['0x70616c69'],
    },
  },
  subql: 'https://subql.darwinia.network/subql-bridger-pangolin',
};

const pangoro = {
  endpoint: {
    websocket: 'wss://pangoro-rpc.darwinia.network',
  },
  name: 'Pangoro',
  logo: 'https://polkadot.js.org/apps/static/pangoro.db0ff9cb..svg',
  color: 'lime',
  explorer: 'https://pangoro.subscan.io',
  currency: 'ORING',
  bridge_target: {
    pangolin: {
      query_name: {
        feemarket: 'pangolinFeeMarket',
        grandpa: 'bridgePangolinGrandpa',
        messages: 'bridgePangolinMessages',
      },
      lanes: ['0x726f6c69'],
    },
  },
  subql: 'https://subql.darwinia.network/subql-bridger-pangoro',
};

const pangolin_parachain = {
  endpoint: {
    websocket: 'wss://pangolin-parachain-rpc.darwinia.network',
  },
  name: 'Pangolin Parachain',
  logo: 'https://polkadot.js.org/apps/static/pangolin.db0518f6..svg',
  color: 'teal',
  explorer: 'https://pangolin-parachain.subscan.io',
  currency: 'PRING',
  bridge_target: {
    pangolin: {
      query_name: {
        feemarket: 'pangolinFeeMarket',
        grandpa: 'bridgePangolinGrandpa',
        messages: 'bridgePangolinMessages',
      },
      lanes: ['0x70616c69'],
    },
  },
  subql: 'https://subql.darwinia.network/subql-bridger-pangolin-parachain',
};

const crab_parachain = {
  endpoint: {
    websocket: 'wss://crab-parachain-rpc.darwinia.network',
  },
  name: 'Crab Parachain',
  logo: 'https://polkadot.js.org/apps/static/crab.ebc98461..svg',
  color: 'deep-purple',
  explorer: 'https://crab-parachain.subscan.io',
  currency: 'CRAB',
  bridge_target: {
    crab: {
      query_name: {
        feemarket: 'crabFeeMarket',
        grandpa: 'bridgePangolinGrandpa',
        messages: 'bridgePangolinMessages',
      },
      lanes: ['0x70616372'],
    }
  },
  subql: 'https://subql.darwinia.network/subql-bridger-crab-parachain',
};

const goerli = {
  endpoint: {
    http: 'https://lodestar-mainnet-rpc.darwinia.network',
  },
  name: 'Goerli',
  logo: 'https://goerli.etherscan.io/images/svg/brands/ethereum.svg?v=1.3',
};


export default {
  darwinia,
  crab,
  pangolin,
  pangoro,
  'pangolinparachain': pangolin_parachain,
  'crabparachain': crab_parachain,
  goerli,
}
