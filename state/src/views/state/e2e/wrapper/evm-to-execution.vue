<template>
  <v-row>
    <v-col cols="12">
      <header-evm-to-execution
        :evm-chain="evmChain"
        :evm-client="evmClient"
        :execution-chain="executionChain"
      />
    </v-col>
    <v-col cols="12">
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
import HeaderEvmToExecution from "@/views/state/e2e/widgets/header-evm-to-execution";

export default {
  components: {HeaderEvmToExecution, E2eFeemarket, MessageRelay},
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
