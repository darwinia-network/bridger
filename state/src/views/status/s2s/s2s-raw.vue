<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <bridge-skeleton
          ref="left_to_right"
          :source-client="source.client.left"
          :source-chain="source.chain.left"
          :target-chain="source.chain.right"
          v-if="source.chain.left && source.chain.right"
        >
          <v-progress-linear
            class="mt-15"
            :color="source.chain.left.color"
            indeterminate
            v-if="loading.sourceClient || loading.targetClient"
          />
          <bridge-basic-s2s
            v-else
            :key="`bridge-${source.chain.left.name}-${source.chain.right.name}`"
            :source-client="source.client.left"
            :target-client="source.client.right"
            :source-chain="source.chain.left"
            :target-chain="source.chain.right"
          />
        </bridge-skeleton>
      </v-col>
      <v-col cols="12">
        <v-divider/>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          ref="right_to_left"
          :source-client="source.client.right"
          :source-chain="source.chain.right"
          :target-chain="source.chain.left"
          v-if="source.chain.left && source.chain.right"
        >
          <v-progress-linear
            class="mt-15"
            :color="source.chain.right.color"
            indeterminate
            v-if="loading.sourceClient || loading.targetClient"
          />
          <bridge-basic-s2s
            v-else
            :key="`bridge-${source.chain.right.name}-${source.chain.left.name}`"
            :source-client="source.client.right"
            :target-client="source.client.left"
            :source-chain="source.chain.right"
            :target-chain="source.chain.left"
          />
        </bridge-skeleton>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>

import {ApiPromise, WsProvider} from '@polkadot/api';
import * as dataSource from '@/data/data_source'

import BridgeSkeleton from '@/views/status/common/bridge-skeleton'
import BridgeBasicS2s from '@/views/status/common/bridge-basic-s2s'

async function initState(vm) {
  const name = vm.bridge.name;
  const [leftName, rightName] = name.split('-');
  const [leftChain, rightChain] = [
    dataSource.chainInfo(leftName),
    dataSource.chainInfo(rightName),
  ];
  if (!leftChain || !rightChain) {
    await vm.$router.push({path: '/'})
    return;
  }
  vm.source.chain.left = {...leftChain, bridge_chain_name: leftName};
  vm.source.chain.right = {...rightChain, bridge_chain_name: rightName};


  vm.loading.sourceClient = true;
  vm.loading.targetClient = true;
  const leftProvider = new WsProvider(vm.source.chain.left.endpoint.websocket);
  const rightProvider = new WsProvider(vm.source.chain.right.endpoint.websocket);
  vm.source.client.left = await ApiPromise.create({provider: leftProvider});
  vm.loading.sourceClient = false;
  vm.$refs['left_to_right'].initState(vm.source.client.left);

  vm.source.client.right = await ApiPromise.create({provider: rightProvider});
  vm.loading.targetClient = false;
  vm.$refs['right_to_left'].initState(vm.source.client.right);

  // window._leftClient = vm.source.client.left;
  // window._rightClient = vm.source.client.right;
}

export default {
  props: {
    bridge: {
      type: Object,
    },
  },
  components: {
    BridgeSkeleton,
    BridgeBasicS2s,
  },
  data: () => ({
    source: {
      client: {
        left: null,
        right: null,
      },
      chain: {
        left: null,
        right: null,
      }
    },
    loading: {
      sourceClient: false,
      targetClient: false,
    },
  }),
  created() {
    initState(this);
  },
  destroyed() {
    const vm = this;
    vm.source.client.left && (vm.source.client.left.disconnect())
    vm.source.client.right && (vm.source.client.right.disconnect())
  }
}
</script>

<style scoped>

</style>
