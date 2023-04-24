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
      <e2e-feemarket
        :source-chain="executionChain"
        :source-client="executionClient"
        :target-chain="evmChain"
      />
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
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>

import {defineProps, onMounted, PropType} from 'vue'
import {EvmClient} from "@/plugins/eth2/evm";
import {ExecutionClient} from "@/plugins/eth2/execution";
import {ConsensusClient} from "@/plugins/eth2/consensus";
import {BridgeEthereumChainInfo} from "@/types/app";
import E2eFeemarket from "@/views/state/e2e/widgets/e2e-feemarket.vue";
import MessageRelay from "@/views/state/e2e/widgets/message-relay.vue";
import CommitteeConsensusToEvm from "@/views/state/e2e/widgets/committee-consensus-to-evm.vue";
import HeaderConsensusToEvm from "@/views/state/e2e/widgets/header-consensus-to-evm.vue";
import HeaderExecutionToEvm from "@/views/state/e2e/widgets/header-execution-to-evm.vue";


const props = defineProps({
  evmChain: {
    type: Object as PropType<BridgeEthereumChainInfo>,
  },
  executionChain: {
    type: Object as PropType<BridgeEthereumChainInfo>,
  },
  consensusChain: {
    type: Object as PropType<BridgeEthereumChainInfo>,
  },
  evmClient: {
    type: Object as PropType<EvmClient>,
  },
  executionClient: {
    type: Object as PropType<ExecutionClient>,
  },
  consensusClient: {
    type: Object as PropType<ConsensusClient>,
  },
});

onMounted(() => {
});


</script>
