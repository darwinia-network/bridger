<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Message</h2>
    </v-col>
    <v-col cols="12" v-for="item in source.lanes" :key="`lane-${item}`">
      <message-relay
        :lane="item"
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
import {onMounted, defineProps, PropType, reactive, toRefs} from 'vue'
import S2sHeaderRelay from "@/views/state/s2s/common/widgets/s2s-header-relay.vue";
import {Bridge} from "@/types/bridge";
import {SubstrateChainInfo} from "@/types/chain";
import {ApiPromise} from "@polkadot/api";
import {BridgeSubstrateChainInfo} from "@/types/app";
import MessageRelay from "@/views/state/s2s/common/widgets/s2s-message-relay/message-relay.vue";

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
});

const state = reactive({
  source: {
    lanes: [],
  }
});

const {source} = toRefs(state);

async function initState() {
  const sourceChainBridgeTarget = props.sourceChain.bridge_target[props.targetChain.bridge_chain_name];
  source.value.lanes = sourceChainBridgeTarget.lanes;
}

onMounted(() => {
  initState();
});

onMounted(() => {
});
</script>
