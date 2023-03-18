<template>
  <v-row>
    <v-col cols="3">
      <span class="ellipsis-text">{{ identity }}</span>
      <v-tooltip activator="parent" location="bottom">
        {{ identity }}
      </v-tooltip>
    </v-col>
    <v-col cols="3">
      <v-btn variant="plain" size="x-small" icon="mdi-content-copy" v-clipboard:copy="identity"></v-btn>
      <v-btn variant="plain" size="x-small" icon="mdi-open-in-new"
             :href="`${chain.explorer}/${type}/${identity}`" target="_blank"
      ></v-btn>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>

import {defineProps, PropType} from 'vue'
import {BridgeSubstrateChainInfo} from "@/types/app";


const props = defineProps({
  identity: {
    type: String,
  },
  type: {
    type: String,
    default: 'block',
  },
  chain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
});

</script>
<style>
.ellipsis-text {
  width: 100%;
  display: block;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
</style>
