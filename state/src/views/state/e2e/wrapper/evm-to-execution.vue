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

<script lang="ts" setup>

import {defineProps, onMounted, PropType} from 'vue'
import {EvmClient} from "@/plugins/eth2/evm";
import {ExecutionClient} from "@/plugins/eth2/execution";
import {ConsensusClient} from "@/plugins/eth2/consensus";
import {BridgeEthereumChainInfo} from "@/types/app";
import HeaderEvmToExecution from "@/views/state/e2e/widgets/header-evm-to-execution.vue";
import E2eFeemarket from "@/views/state/e2e/widgets/e2e-feemarket.vue";
import MessageRelay from "@/views/state/e2e/widgets/message-relay.vue";


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
