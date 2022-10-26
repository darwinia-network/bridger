<template>
  <v-row>
    <v-col cols="12">
      <committee-consensus-to-evm
        :evm-chain="evmChain"
        :execution-chain="executionChain"
        :consensus-chain="consensusChain"
        :evm-client="evmClient"
        :execution-client="executionClient"
        :consensus-client="consensusClient"
      />
    </v-col>
    <v-col cols="12">
      <header-consensus-to-evm
        :evm-chain="evmChain"
        :execution-chain="executionChain"
        :consensus-chain="consensusChain"
        :evm-client="evmClient"
        :execution-client="executionClient"
        :consensus-client="consensusClient"
      />
    </v-col>
    <v-col cols="12">
      <header-execution-to-evm
        :evm-chain="evmChain"
        :execution-chain="executionChain"
        :consensus-chain="consensusChain"
        :evm-client="evmClient"
        :execution-client="executionClient"
        :consensus-client="consensusClient"
      />
    </v-col>
  </v-row>
</template>

<script>
import CommitteeConsensusToEvm from '@/views/state/e2e/widgets/committee-consensus-to-evm';
import HeaderConsensusToEvm from '@/views/state/e2e/widgets/header-consensus-to-evm';
import HeaderExecutionToEvm from "@/views/state/e2e/widgets/header-execution-to-evm";

export default {
  components: {HeaderExecutionToEvm, HeaderConsensusToEvm, CommitteeConsensusToEvm},
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
  data: () => ({

  }),
  created() {
  }
}
</script>

<style scoped>

</style>
