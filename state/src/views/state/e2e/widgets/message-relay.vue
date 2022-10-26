<template>
  <v-row>
    <v-col cols="12">
      message relay
      <pre>{{ source }}</pre>
      <pre>{{ loading }}</pre>
    </v-col>
  </v-row>
</template>

<script>

async function initState(vm) {
  const bridgeTargetAtSource = vm.sourceChain.bridge_target[vm.targetChain.bridge_chain_name];
  const bridgeTargetAtTarget = vm.targetChain.bridge_target[vm.sourceChain.bridge_chain_name];

  vm.loading.targetChainInboundLaneData = true;
  const inboundLaneNonce = await vm.targetClient.message({
    inbound: bridgeTargetAtTarget.contract.inbound,
    outbound: bridgeTargetAtTarget.contract.outbound,
  }).inboundLaneNonce();
  vm.source.targetChainInboundLaneData = {
    last_confirmed_nonce: inboundLaneNonce.last_confirmed_nonce,
    last_delivered_nonce: inboundLaneNonce.last_delivered_nonce,
    relayer_range_back: inboundLaneNonce.relayer_range_back,
    relayer_range_front: inboundLaneNonce.relayer_range_front,
  };
  vm.loading.targetChainInboundLaneData = false;

  vm.loading.sourceChainOutboundLaneData = true;
  const outboundLaneNonce = await vm.sourceClient.message({
    inbound: bridgeTargetAtSource.contract.inbound,
    outbound: bridgeTargetAtSource.contract.outbound,
  }).outboundLaneNonce();
  vm.source.sourceChainOutboundLaneData = {
    latest_generated_nonce: outboundLaneNonce.latest_generated_nonce,
    latest_received_nonce: outboundLaneNonce.latest_received_nonce,
    oldest_unpruned_nonce: outboundLaneNonce.oldest_unpruned_nonce,
  };
  vm.loading.sourceChainOutboundLaneData = false;
}

export default {
  props: {
    sourceChain: {
      type: Object,
    },
    targetChain: {
      type: Object,
    },
    sourceClient: {
      type: Object,
    },
    targetClient: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      sourceChainOutboundLaneData: null,
      targetChainInboundLaneData: null,
    },
    loading: {
      sourceChainOutboundLaneData: true,
      targetChainInboundLaneData: true,
    }
  }),
  created() {
    initState(this)
  }
}
</script>

<style scoped>

</style>
