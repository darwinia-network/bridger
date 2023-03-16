<template>
  <v-row>
    <v-col md="6" sm="12" xs="12">
      <s2s-header-relay
        :key="`s2s-basic-header-${props.sourceChain.name}-${props.targetChain.name}`"
        :source-chain="props.sourceChain"
        :source-client="props.sourceClient"
        :target-chain="props.targetChain"
        :target-client="props.targetClient"
      />
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>
import {onMounted, defineProps, PropType, reactive, toRefs} from 'vue'
import S2sHeaderRelay from "@/views/state/s2s/common/widgets/s2s-header-relay.vue";
import {Bridge} from "@/types/bridge";
import {SubstrateChainInfo} from "@/types/chain";
import {ApiPromise} from "@polkadot/api";

const props = defineProps({
  parachainBridge: {
    type: Boolean,
    default: false,
  },
  sourceChain: {
    type: Object as PropType<SubstrateChainInfo>,
  },
  sourceClient: {
    type: Object as PropType<ApiPromise>,
  },
  targetChain: {
    type: Object as PropType<SubstrateChainInfo>,
  },
  targetClient: {
    type: Object as PropType<ApiPromise>,
  },
})

onMounted(() => {
});
</script>
