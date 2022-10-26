<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <bridge-skeleton
          ref="left_to_right"
          chain-type="execution"
          :key="`e2e-${source.chain.evm.name}-${source.chain.execution.name}`"
          :source-client="source.client.evm"
          :source-chain="source.chain.evm"
          :target-chain="source.chain.execution"
          v-if="source.chain.evm && source.chain.execution"
        >
          <v-progress-linear
            class="mt-15"
            :color="source.chain.evm.color"
            indeterminate
            v-if="loading.evmClient || loading.executionClient"
          />
        </bridge-skeleton>
      </v-col>
      <v-col cols="12">
        <v-divider/>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          ref="right_to_left"
          chain-type="execution"
          :key="`e2e-${source.chain.execution.name}-${source.chain.evm.name}`"
          :source-client="source.client.execution"
          :source-chain="source.chain.execution"
          :target-chain="source.chain.evm"
          v-if="source.chain.evm && source.chain.execution"
        >
          <v-progress-linear
            class="mt-15"
            :color="source.chain.execution.color"
            indeterminate
            v-if="loading.evmClient || loading.executionClient"
          />
          <execution-to-evm
            v-else
            :evm-chain="source.chain.evm"
            :execution-chain="source.chain.execution"
            :consensus-chain="source.chain.consensus"
            :evm-client="source.client.evm"
            :execution-client="source.client.execution"
            :consensus-client="source.client.consensus"
          />
        </bridge-skeleton>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>

import BridgeSkeleton from '@/components/skeleton/bridge-skeleton';
import ExecutionToEvm from '@/views/state/e2e/wrapper/execution-to-evm';

import * as dataSource from '@/data/data_source';

async function initState(vm) {
  const name = vm.bridge.name;
  const [evmChainName, executionChainName] = name.split('-');
  const [evmChain, executionChain] = [
    dataSource.chainInfo(evmChainName),
    dataSource.chainInfo(executionChainName),
  ];
  if (!evmChain || !executionChain) {
    await vm.$router.push({path: '/'})
    return;
  }
  const consensusChain = dataSource.chainInfo(executionChain.consensus_chain);
  vm.source.chain.evm = {...evmChain, bridge_chain_name: evmChainName};
  vm.source.chain.execution = {...executionChain, bridge_chain_name: executionChainName};
  vm.source.chain.consensus = {...consensusChain, bridge_chain_name: executionChain.consensus};
  vm.source.client.evm = vm.$eth2.evm({endpoint: vm.source.chain.evm.endpoint.evm});
  vm.source.client.execution = vm.$eth2.execution({endpoint: vm.source.chain.execution.endpoint.http});
  vm.source.client.consensus = vm.$eth2.consensus({endpoint: vm.source.chain.consensus.endpoint.http});
  vm.loading.evmClient = false;
  vm.loading.executionClient = false;
  vm.loading.consensusClient = false;
}

export default {
  components: {ExecutionToEvm, BridgeSkeleton},
  props: {
    bridge: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      chain: {
        evm: null,
        execution: null,
        consensus: null,
      },
      client: {
        evm: null,
        execution: null,
        consensus: null,
      },
    },
    loading: {
      evmClient: true,
      executionClient: true,
      consensusClient: true,
    }
  }),
  created() {
    initState(this);
  }
}
</script>

<style scoped>

</style>
