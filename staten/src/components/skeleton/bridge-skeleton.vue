<template>
  <v-row v-if="chainType !== 'substrate' || (source.lastBlock && source.finalizedBlock)">
    <v-col cols="12" md="2" class="pt-8" v-if="sourceChain">
      <v-row>
        <v-col cols="12" class="d-flex flex-column align-center">
          <v-avatar size="64" v-ripple>
            <img :src="sourceChain.logo" :alt="sourceChain.name" v-if="sourceChain.logo">
            <v-icon v-if="!sourceChain.logo" size="54">mdi-alpha-c</v-icon>
          </v-avatar>
          <span class="body-1 font-weight-light">
            <span v-text="sourceChain.name"/>
          </span>
        </v-col>
        <v-col cols="12" class="text-center">
          <p class="title text-block-number" v-if="source.lastBlock">
            <span v-text="source.lastBlock.number"/>
            <v-btn icon size="x-small" variant="plain" :href="`${sourceChain.explorer}/block/${source.lastBlock.number}`" target="_blank">
              <v-icon size="x-small">mdi-open-in-new</v-icon>
            </v-btn>
          </p>
          <p class="subtitle-1 text-block-number" v-if="source.finalizedBlock">
            <span v-text="source.finalizedBlock.number"/>
            <v-btn icon size="x-small" variant="plain" :href="`${sourceChain.explorer}/block/${source.finalizedBlock.number}`"
                   target="_blank">
              <v-icon size="x-small">mdi-open-in-new</v-icon>
            </v-btn>
          </p>
        </v-col>
      </v-row>
    </v-col>
    <v-col cols="12" md="8">
      <slot/>
    </v-col>

    <v-col cols="12" md="2" class="pt-8" v-if="targetChain">
      <v-row>
        <v-col cols="12" class="d-flex flex-column align-center">
          <v-avatar size="64" v-ripple>
            <img :src="targetChain.logo" :alt="targetChain.name" v-if="targetChain.logo">
            <v-icon v-if="!targetChain.logo" size="54">mdi-alpha-c</v-icon>
          </v-avatar>
          <span class="body-1 font-weight-light" v-text="targetChain.name"></span>
        </v-col>
      </v-row>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>

import {defineProps, onMounted, PropType, reactive, toRefs} from 'vue'
import {BridgeSubstrateChainInfo} from "@/types/app";
import {ApiPromise} from "@polkadot/api";
import EllipsisText from "@/components/widgets/ellipsis-text.vue";


const props = defineProps({
  chainType: {
    type: String,
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
  grandpaPalletName: {
    type: String,
  },
});

const state = reactive({
  source: {
    lastBlock: null,
    finalizedBlock: null,
  }
});

const {source} = toRefs(state);

async function initState() {
  if (props.chainType == 'substrate') {
    props.sourceClient.rpc.chain.subscribeNewHeads(header => {
      source.value.lastBlock = header.toJSON();
    });
    props.sourceClient.rpc.chain.subscribeFinalizedHeads(header => {
      source.value.finalizedBlock = header.toJSON();
    });
  }
}


onMounted(() => {
  initState()
});
</script>
