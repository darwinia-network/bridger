<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <bridge-skeleton
          ref="left_to_right"
          chain-type="ethereum"
          :key="`e2e-${source.chain.evm.name}-${source.chain.ethereum.name}`"
          :source-client="source.client.evm"
          :source-chain="source.chain.evm"
          :target-chain="source.chain.ethereum"
          v-if="source.chain.evm && source.chain.ethereum"
        >
          <v-progress-linear
            class="mt-15"
            :color="source.chain.evm.color"
            indeterminate
            v-if="loading.evmClient || loading.ethereumClient"
          />
        </bridge-skeleton>
      </v-col>
      <v-col cols="12">
        <v-divider/>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          ref="right_to_left"
          chain-type="ethereum"
          :key="`e2e-${source.chain.ethereum.name}-${source.chain.evm.name}`"
          :source-client="source.client.ethereum"
          :source-chain="source.chain.ethereum"
          :target-chain="source.chain.evm"
          v-if="source.chain.evm && source.chain.ethereum"
        >
          <v-progress-linear
            class="mt-15"
            :color="source.chain.ethereum.color"
            indeterminate
            v-if="loading.evmClient || loading.ethereumClient"
          />
        </bridge-skeleton>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>

import BridgeSkeleton from '@/components/skeleton/bridge-skeleton';

import Web3 from 'web3'

import * as dataSource from '@/data/data_source';

async function initState(vm) {
  const name = vm.bridge.name;
  const [evmChainName, ethereumChainName] = name.split('-');
  const [evmChain, ethereumChain] = [
    dataSource.chainInfo(evmChainName),
    dataSource.chainInfo(ethereumChainName),
  ];
  if (!evmChain || !ethereumChain) {
    await vm.$router.push({path: '/'})
    return;
  }
  vm.source.chain.evm = {...evmChain, bridge_chain_name: evmChainName};
  vm.source.chain.ethereum = {...ethereumChain, bridge_chain_name: ethereumChainName};
  vm.source.client.evm = new Web3(vm.source.chain.evm.endpoint.evm);
  vm.source.client.ethereum = new Web3(vm.source.chain.ethereum.endpoint.http);
  console.log(vm.source.client.evm);
  vm.loading.evmClient = false;
  vm.loading.ethereumClient = false;
}

export default {
  components: {BridgeSkeleton},
  props: {
    bridge: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      chain: {
        evm: null,
        ethereum: null,
      },
      client: {
        evm: null,
        ethereum: null,
      },
    },
    loading: {
      evmClient: true,
      ethereumClient: true,
    }
  }),
  created() {
    initState(this);
  }
}
</script>

<style scoped>

</style>
