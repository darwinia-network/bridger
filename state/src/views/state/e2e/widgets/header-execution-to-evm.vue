<template>
  <v-row>
    <v-col cols="12">
      execution -> evm
    </v-col>
  </v-row>
</template>

<script>

async function initState(vm) {
  const bridgeTarget = vm.evmChain.bridge_target[vm.executionChain.bridge_chain_name];
  const consensusLightClientAddress = bridgeTarget.contract.lc_consensus;

}

export default {
  props: {
    evmChain: {
      type: Object,
    },
    executionChain: {
      type: Object,
    },
    consensusChain: {
      type: Object,
    },
    evmClient: {
      type: Object,
    },
    executionClient: {
      type: Object,
    },
    consensusClient: {
      type: Object,
    },
  },
  data: () => ({}),
  created() {
    initState(this);
  }
}
</script>

<style scoped>

</style>
