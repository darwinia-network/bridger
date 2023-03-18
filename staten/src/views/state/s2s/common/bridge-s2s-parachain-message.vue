<template>
  <v-row>
    <v-col cols="12">
      <s2s-message-relay
        :key="`s2s-message-${paraChain.name}-${targetChain.name}`"
        :parachain-bridge="false"
        :source-client="paraClient"
        :target-client="targetClient"
        :source-chain="paraChain"
        :target-chain="targetChain"
      />
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>
import S2sMessageRelay from '@/views/state/s2s/common/widgets/s2s-message-relay/index.vue'
import {defineProps, onMounted, PropType} from 'vue'
import {ApiPromise} from "@polkadot/api";
import {BridgeSubstrateChainInfo} from "@/types/app";

const props = defineProps({
  relayChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  relayClient: {
    type: Object as PropType<ApiPromise>,
  },
  paraChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  paraClient: {
    type: Object as PropType<ApiPromise>,
  },
  targetChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  targetClient: {
    type: Object as PropType<ApiPromise>,
  },
});

async function initState() {
  //
}

onMounted(() => {
  initState();
});

onMounted(() => {
});
</script>
