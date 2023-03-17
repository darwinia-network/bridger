<template>
  <v-row>
    <v-col md="6" sm="12" xs="12">
      <parachain-header-relay
        :relay-client="relayClient"
        :relay-chain="relayChain"
        :para-client="paraClient"
        :para-chain="paraChain"
        :target-chain="targetChain"
        :target-client="targetClient"
      />
    </v-col>
    <v-col md="6" sm="12" xs="12">
      <parahead-relay
        :relay-chain="relayChain"
        :relay-client="relayClient"
        :para-chain="paraChain"
        :para-client="paraClient"
        :target-chain="targetChain"
        :target-client="targetClient"
      />
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>

import {defineProps, inject, onBeforeUnmount, onMounted, PropType, reactive, toRefs} from 'vue'
import {BridgeSubstrateChainInfo} from "@/types/app";
import {ApiPromise} from "@polkadot/api";
import ParachainHeaderRelay from "@/views/state/s2s/common/widgets/parachain-header-relay.vue";
import ParaheadRelay from "@/views/state/s2s/common/widgets/parahead-relay.vue";

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

</script>
