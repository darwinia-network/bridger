<template>
  <v-row>
    <template v-if="status_bridge.group === 'S2S'">
      <v-col
        cols="12"
        v-if="status_bridge.bridge.bridge_type === 'para-with-para'"
      >
        <para-with-para :key="status_bridge.bridge.name" :bridge="status_bridge.bridge"/>
      </v-col>
    </template>

    <template v-if="status_bridge.group === 'E2E'">
      <v-col cols="12">
        <e2e-raw :key="status_bridge.bridge.name" :bridge="status_bridge.bridge"/>
      </v-col>
    </template>

  </v-row>
</template>

<script lang="ts" setup>

import ParaWithPara from "@/views/state/s2s/para-with-para.vue";

import {onMounted, reactive, toRefs, watch} from 'vue'
import {useRoute, useRouter} from 'vue-router'

import * as dataSource from "@/data/data_source";
import {BridgeInfo} from "@/types/bridge";
import E2eRaw from "@/views/state/e2e/e2e-raw.vue";

const route = useRoute();
const router = useRouter();


const state = reactive({
  status_bridge: {} as BridgeInfo,
});


const {
  status_bridge,
} = toRefs(state);

watch(
  () => route.path,
  async () => {
    initState();
  }
)

function initState() {
  const params = route.params
  const bridgeInfo = dataSource.findBridge(params.bridge.toString());
  if (!bridgeInfo) {
    router.push({path: '/'});
    return;
  }
  status_bridge.value = bridgeInfo;
}


onMounted(() => {
  initState();
});
</script>
