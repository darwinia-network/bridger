<template>
  <v-row>
    <template v-if="source.status_bridge.group === 'S2S'">
      <v-col
        cols="12"
        v-if="source.status_bridge.bridge.bridge_type === 's2s'"
      >
        <s2s-raw :key="source.status_bridge.bridge.name" :bridge="source.status_bridge.bridge"/>
      </v-col>
      <v-col
        cols="12"
        v-if="source.status_bridge.bridge.bridge_type === 'parachain'"
      >
        <s2s-parachain :key="source.status_bridge.bridge.name" :bridge="source.status_bridge.bridge"/>
      </v-col>
    </template>

    <template v-if="source.status_bridge.group === 'E2E'">
      <v-col cols="12">
        <e2e-raw :key="source.status_bridge.bridge.name" :bridge="source.status_bridge.bridge"/>
      </v-col>
    </template>

    <v-col cols="12">
      <v-container>
        <v-alert text color="grey accent-4" icon="mdi-information">
          More feemarket operation please visit
          <v-btn text small href="https://feemarket.darwinia.network" target="_blank">
            feemarket ui
          </v-btn>
          project.
        </v-alert>
      </v-container>
    </v-col>
  </v-row>
</template>

<script>

import * as dataSource from '@/data/data_source.js'

import S2sRaw from '@/views/state/s2s/s2s-raw';
import S2sParachain from '@/views/state/s2s/s2s-parachain';
import E2eRaw from "@/views/state/e2e/e2e-raw";

function initState(vm) {
  const params = vm.$route.params;
  const gbridge = dataSource.findBridge(params.bridge);
  if (!gbridge) {
    vm.$router.push({path: '/'});
    return;
  }
  vm.source.status_bridge = gbridge;
}

export default {
  components: {
    E2eRaw,
    S2sParachain,
    S2sRaw,
  },
  data: () => ({
    source: {
      status_bridge: null,
    },
  }),
  watch: {
    '$route.path': {
      handler() {
        initState(this);
      },
      deep: true,
    }
  },
  created() {
    const vm = this;
    initState(vm);
  }
}
</script>

<style scoped>

</style>
