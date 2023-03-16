<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Header</h2>
    </v-col>
    <v-col cols="12">
      <v-table>
        <thead>
        <tr>
          <th style="width: 45%">Title</th>
          <th>Value</th>
        </tr>
        </thead>
        <tbody>
        <tr>
          <td class="subtitle-2">Last relayed block (hash)</td>
          <td>
            <!--            v-if="loading.bestFinalizedHash"-->
            <v-progress-linear :color="sourceChain.color" indeterminate/>
            <!--            <external-explorer
                          v-else
                          :identity="source.bestFinalizedHash"
                          type="block"
                          :chain="sourceChain"
                        />-->
          </td>
        </tr>
        </tbody>
      </v-table>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>
import {onMounted, defineProps, PropType, reactive, toRefs} from 'vue'
import {SubstrateChainInfo} from "@/types/chain";
import {ApiPromise} from "@polkadot/api";

const props = defineProps({
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
