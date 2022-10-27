<template>
  <v-row>
    <v-col md="6" sm="12" xs="12">
      <e2e-feemarket
        :source-chain="evmChain"
        :source-client="evmClient"
        :target-chain="executionChain"
      />
    </v-col>
    <v-col cols="12">
      <message-relay
        direction="evm-to-execution"
        :source-chain="evmChain"
        :target-chain="executionChain"
        :source-client="evmClient"
        :target-client="executionClient"
        :consensus-client="consensusClient"
      />
    </v-col>
  </v-row>
</template>

<script>
import MessageRelay from "@/views/state/e2e/widgets/message-relay";
import E2eFeemarket from "@/views/state/e2e/widgets/e2e-feemarket";

export default {
  components: {E2eFeemarket, MessageRelay},
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
  }
}
</script>

<style scoped>

</style>
