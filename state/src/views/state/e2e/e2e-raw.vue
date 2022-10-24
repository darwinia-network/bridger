<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <bridge-skeleton
          ref="left_to_right"
          chain-type="ethereum"
          :key="`e2e-${source.chain.left.name}-${source.chain.right.name}`"
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
        </bridge-skeleton>
      </v-col>
      <v-col cols="12">
        <v-divider/>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          ref="right_to_left"
          chain-type="ethereum"
          :key="`e2e-${source.chain.right.name}-${source.chain.left.name}`"
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
        </bridge-skeleton>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>

import * as dataSource from "@/data/data_source";
import BridgeSkeleton from "@/components/skeleton/bridge-skeleton";

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
        left: null,
        right: null,
      },
      client: {
        left: null,
        right: null,
      },
    },
    loading: {
      sourceClient: true,
      targetClient: true,
    }
  }),
  created() {
    initState(this);
  }
}
</script>

<style scoped>

</style>
