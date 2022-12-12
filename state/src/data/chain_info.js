const darwinia = {
  endpoint: {
    websocket: 'wss://rpc.darwinia.network',
    evm: 'https://rpc.darwinia.network',
  },
  name: 'Darwinia',
  logo: 'https://polkadot.js.org/apps/static/darwinia.70c5ca41..svg',
  color: 'pink',
  explorer: 'https://darwinia.subscan.io',
  currency: 'RING',
  precision: 9,
  precisionEvm: 18,
  bridge_target: {
    crab: {
      query_name: {
        feemarket: 'feeMarket',
        grandpa: 'bridgeCrabGrandpa',
        messages: 'bridgeCrabMessages',
      },
      lanes: ['0x00000000'],
    },
    ethereum: {
      contract: {
        lc_consensus: '0xD2A37C4523542F2dFD7Cb792D2aeAd5c61C1bAAE',
        lc_execution: '0xeC3c9B4d3674B3D03fdf20b082A3C2c669075990',
        inbound: '0xf1B8a9F8436800499DB8186f2da2fb3e78Ff7c2B',
        outbound: '0xcA3749C8C3aF04278D596a3fBe461481B6aa1b01',
        feemarket: '0xcA927Df15afb7629b79dA4713a871190315c7409',
        chain_message_committer: '0xea7d0fE22AF25d9655f7AdDd6a59E37e84B3AB5F',
        lane_message_committer: '0x56746a8099a7e6D962802A23e01FeDdc1282cDAe',
      },
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
  color: '#bbbbbb',
  explorer: 'https://goerli.etherscan.io',
  currency: 'ETH',
  precision: 18,
};

const ethereum = {
  endpoint: {
    http: 'https://eth-mainnet.g.alchemy.com/v2/x-Yy-QAog0fzfJQlr0VKreOZaq4KgVUJ',
  },
  name: 'Ethereum',
  logo: 'https://cdn.worldvectorlogo.com/logos/ethereum-eth.svg',
  color: '#627eea',
  explorer: 'https://etherscan.io',
  currency: 'ETH',
  precision: 18,
  consensus_chain: 'lodestarmainnet',
  bridge_target: {
    darwinia: {
      contract: {
        inbound: '0x4E210866d089856a8A0435965FefEe19640487E5',
        outbound: '0x169F28bfbfFCddFdc772A94Cf020bbB4CAdc8E01',
        feemarket: '0xCD97185B7d05f8ea91d241C8dfD51a2Cc9c0547a',
        posa: '0xf46349a32cA70C0B9fFbD19937Fb1623e7F3db19',
      },
    }
  }
};

const lodestarMainnet = {
  endpoint: {
    http: 'https://lodestar-mainnet-rpc.darwinia.network',
  },
  name: 'Lodestar',
  logo: 'https://chainsafe.github.io/lodestar/assets/lodestar_icon_300.png',
  color: '#ce8596',
  explorer: null,
  currency: 'ETH',
  precision: 18,
};


export default {
  darwinia,
  crab,
  pangolin,
  pangoro,
  'pangolinparachain': pangolin_parachain,
  'pangolinparachainalpha': pangolin_parachain_alpha,
  'crabparachain': crab_parachain,
  kusama,
  rococo,
  moonbase,
  goerli,
  ethereum,
  lodestarmainnet: lodestarMainnet,
}
