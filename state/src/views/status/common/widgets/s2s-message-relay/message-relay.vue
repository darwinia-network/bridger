<template>
  <v-row>
    <v-col cols="12">
      <pre>{{ source.outboundLaneData }}</pre>
    </v-col>
  </v-row>
</template>

<script>


async function initState(vm) {
  const sourceChainBridgeTarget = vm.sourceChain.bridge_target[vm.targetChain.bridge_chain_name];
  vm.subscriber.outboundLanes = await vm.sourceClient.query[sourceChainBridgeTarget.query_name.messages]
    .outboundLanes(vm.lane, async v => {
      vm.source.outboundLaneData = v.toJSON();
    });
}

export default {
  props: {
    lane: {
      type: String,
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
  components: {},
  data: () => ({
    source: {
      outboundLaneData: null,
    },
    subscriber: {
      outboundLanes: null,
    },
  }),
  created() {
    initState(this);
  },
  destroyed() {
    const vm = this;
    vm.subscriber.outboundLanes && vm.subscriber.outboundLanes();
  }
}
</script>

<style scoped>

</style>
