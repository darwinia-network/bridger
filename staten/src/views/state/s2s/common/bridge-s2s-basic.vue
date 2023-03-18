<template>
  <v-row>
    <v-col md="6" sm="12" xs="12">
      <s2s-header-relay
        v-if="props.sourceClient && props.targetClient"
        :key="`s2s-basic-header-${props.sourceChain.name}-${props.targetChain.name}`"
        :source-chain="props.sourceChain"
        :source-client="props.sourceClient"
        :target-chain="props.targetChain"
        :target-client="props.targetClient"
      />
    </v-col>
    <v-col md="6" sm="12" xs="12">
      <s2s-feemarket
        :key="`s2s-feemarket-${sourceChain.name}-${targetChain.name}`"
        :source-client="sourceClient"
        :source-chain="sourceChain"
        :target-chain="targetChain"
      />
    </v-col>
    <v-col cols="12">
      <s2s-message-relay
        :key="`s2s-message-${sourceChain.name}-${targetChain.name}`"
        :parachain-bridge="parachainBridge"
        :source-client="sourceClient"
        :target-client="targetClient"
        :source-chain="sourceChain"
        :target-chain="targetChain"
      />
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>
import {defineProps, onMounted, PropType} from 'vue'
import S2sHeaderRelay from "@/views/state/s2s/common/widgets/s2s-header-relay.vue";
import {ApiPromise} from "@polkadot/api";
import {BridgeSubstrateChainInfo} from "@/types/app";
import S2sFeemarket from "@/views/state/s2s/common/widgets/s2s-feemarket.vue";
import S2sMessageRelay from "@/views/state/s2s/common/widgets/s2s-message-relay/index.vue";

const props = defineProps({
  parachainBridge: {
    type: Boolean,
    default: false,
  },
  sourceChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  sourceClient: {
    type: Object as PropType<ApiPromise>,
  },
  targetChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  targetClient: {
    type: Object as PropType<ApiPromise>,
  },
})

onMounted(() => {
});
</script>
