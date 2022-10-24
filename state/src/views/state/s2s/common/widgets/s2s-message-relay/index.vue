<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Message</h2>
    </v-col>
    <v-col cols="12" v-for="item in source.lanes" :key="`lane-${item}`">
      <message-relay
        :lane="item"
        :parachain-bridge="parachainBridge"
        :source-client="sourceClient"
        :target-client="targetClient"
        :source-chain="sourceChain"
        :target-chain="targetChain"
      />
    </v-col>
  </v-row>
</template>

<script>

import MessageRelay from './message-relay'

async function initState(vm) {
  const sourceChainBridgeTarget = vm.sourceChain.bridge_target[vm.targetChain.bridge_chain_name];
  vm.source.lanes = sourceChainBridgeTarget.lanes;
}

export default {
  components: {MessageRelay},
  props: {
    parachainBridge: {
      type: Boolean,
      default: false,
    },
    sourceClient: {
      type: Object,
    },
    targetClient: {
      type: Object,
    },
    sourceChain: {
      type: Object,
    },
    targetChain: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      lanes: [],
    },
  }),
  created() {
    initState(this);
  },
}
</script>

<style scoped>

</style>
