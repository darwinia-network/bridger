const darwinia = {
  endpoint: {
    websocket: 'wss://rpc.darwinia.network',
  },
  name: 'Darwinia',
  logo: 'https://polkadot.js.org/apps/static/darwinia.70c5ca41..svg',
  color: 'pink',
  explorer: 'https://darwinia.subscan.io',
  currency: 'RING',
  precision: 9,
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
  precision: 9,
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
      para_id: 2105,
      relay_chain: 'kusama',
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
  precision: 9,
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
        parachains: 'bridgeRococoParachains',
      },
      lanes: ['0x70616c69'],
      para_id: 2105,
      relay_chain: 'rococo',
    },
    pangolinparachainalpha: {
      query_name: {
        feemarket: 'pangolinParachainAlphaFeeMarket',
        grandpa: 'bridgeMoonbaseRelayGrandpa',
        messages: 'bridgePangolinParachainAlphaMessages',
        parachains: 'bridgeMoonbaseRelayParachains',
      },
      lanes: ['0x706c7061'],
      para_id: 2105,
      relay_chain: 'moonbase',
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
  precision: 9,
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
  precision: 18,
  bridge_target: {
    pangolin: {
      query_name: {
        feemarket: 'pangolinFeeMarket',
        grandpa: 'bridgePangolinGrandpa',
        messages: 'bridgePangolinMessages',
      },
      lanes: ['0x70616c69'],
      para_id: 2105,
      relay_chain: 'rococo',
    },
  },
  subql: 'https://subql.darwinia.network/subql-bridger-pangolin-parachain',
};

const pangolin_parachain_alpha = {
  endpoint: {
    websocket: 'wss://pangolin-parachain-alpha-rpc.darwinia.network',
  },
  name: 'Pangolin Parachain Alpha',
  logo: 'https://polkadot.js.org/apps/static/pangolin.db0518f6..svg',
  color: 'teal',
  explorer: null,
  currency: 'PRING',
  precision: 18,
  bridge_target: {
    pangolin: {
      query_name: {
        feemarket: 'pangolinFeeMarket',
        grandpa: 'bridgePangolinGrandpa',
        messages: 'bridgePangolinMessages',
      },
      lanes: ['0x706c7061'],
      para_id: 2105,
      relay_chain: 'moonbase',
    },
  },
  subql: 'https://subql.darwinia.network/subql-bridger-pangolin-parachainalpha',
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
  precision: 18,
  bridge_target: {
    crab: {
      query_name: {
        feemarket: 'crabFeeMarket',
        grandpa: 'bridgeCrabGrandpa',
        messages: 'bridgeCrabMessages',
      },
      lanes: ['0x70616372'],
      para_id: 2105,
      relay_chain: 'kusama',
    }
  },
  subql: 'https://subql.darwinia.network/subql-bridger-crab-parachain',
};

const kusama = {
  endpoint: {
    websocket: 'wss://kusama-rpc.polkadot.io',
  },
  name: 'Kusama',
  logo: 'https://polkadot.js.org/apps/static/kusama-128.e5f13822..gif',
  color: '#424242',
  explorer: 'https://kusama.subscan.io',
  currency: 'KSM',
  precision: 9,
  subql: 'https://subql.darwinia.network/subql-bridger-kusama',
};

const rococo = {
  endpoint: {
    websocket: 'wss://rococo-rpc.polkadot.io',
  },
  name: 'Rococo',
  logo: 'https://polkadot.js.org/apps/static/rococo.560c473b..svg',
  color: 'purple',
  explorer: 'https://rococo.subscan.io',
  currency: 'ROC',
  precision: 9,
  subql: 'https://subql.darwinia.network/subql-bridger-rococo',
};

const moonbase = {
  endpoint: {
    websocket: 'wss://frag-moonbase-relay-rpc-ws.g.moonbase.moonbeam.network',
  },
  name: 'Moonbase',
  logo: 'https://polkadot.js.org/apps/static/polkadot-circle.1eea41b2..svg',
  color: '#FF4081',
  explorer: null,
  currency: 'M',
  precision: 9,
  subql: 'https://subql.darwinia.network/subql-bridger-moonbase',
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
  'pangolinparachainalpha': pangolin_parachain_alpha,
  'crabparachain': crab_parachain,
  goerli,
  kusama,
  rococo,
  moonbase,
}
