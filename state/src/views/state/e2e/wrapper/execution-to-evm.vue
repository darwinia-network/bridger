<template>
  <v-row>
    <v-col md="6" sm="12" xs="12">
      <committee-consensus-to-evm
        :evm-chain="evmChain"
        :execution-chain="executionChain"
        :consensus-chain="consensusChain"
        :evm-client="evmClient"
        :execution-client="executionClient"
        :consensus-client="consensusClient"
      />
    </v-col>
    <v-col md="6" sm="12" xs="12">
      <e2e-feemarket/>
    </v-col>
    <v-col md="6" sm="12" xs="12">
      <header-consensus-to-evm
        :evm-chain="evmChain"
        :execution-chain="executionChain"
        :consensus-chain="consensusChain"
        :evm-client="evmClient"
        :execution-client="executionClient"
        :consensus-client="consensusClient"
      />
    </v-col>
    <v-col md="6" sm="12" xs="12">
      <header-execution-to-evm
        :evm-chain="evmChain"
        :execution-chain="executionChain"
        :consensus-chain="consensusChain"
        :evm-client="evmClient"
        :execution-client="executionClient"
        :consensus-client="consensusClient"
      />
    </v-col>
    <v-col cols="12">
      <message-relay
        direction="execution-to-evm"
        :source-chain="executionChain"
        :target-chain="evmChain"
        :source-client="executionClient"
        :target-client="evmClient"
      />
      <!--      <message-relay
              direction="evm-to-execution"
              :source-chain="evmChain"
              :target-chain="executionChain"
              :source-client="evmClient"
              :target-client="executionClient"
              :consensus-client="consensusClient"
            />-->
    </v-col>
  </v-row>
</template>

<script>
import CommitteeConsensusToEvm from '@/views/state/e2e/widgets/committee-consensus-to-evm';
import HeaderConsensusToEvm from '@/views/state/e2e/widgets/header-consensus-to-evm';
import HeaderExecutionToEvm from "@/views/state/e2e/widgets/header-execution-to-evm";
import MessageRelay from "@/views/state/e2e/widgets/message-relay";
import E2eFeemarket from "@/views/state/e2e/widgets/e2e-feemarket";

export default {
  components: {E2eFeemarket, MessageRelay, HeaderExecutionToEvm, HeaderConsensusToEvm, CommitteeConsensusToEvm},
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
