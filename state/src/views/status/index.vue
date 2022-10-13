<template>
  <v-row>
    <v-col
      cols="12"
      v-if="source.status_bridge.group = 'S2S' && source.status_bridge.bridge.bridge_type === 's2s'"
    >
      <s2s-raw :key="source.status_bridge.bridge.name" :bridge="source.status_bridge.bridge"/>
    </v-col>
    <v-col
      cols="12"
      v-if="source.status_bridge.group = 'S2S' && source.status_bridge.bridge.bridge_type === 'parachain'"
    >
      <s2s-parachain :key="source.status_bridge.bridge.name" :bridge="source.status_bridge.bridge"/>
    </v-col>
  </v-row>
</template>

<script>

import * as dataSource from '@/data/data_source.js'

import S2sRaw from '@/views/status/s2s/s2s-raw';
import S2sParachain from '@/views/status/s2s/s2s-parachain';

function initState(vm) {
  const params = vm.$route.params;
  const gbridge = dataSource.findBridge(params.bridge);
  console.log(JSON.stringify(gbridge));
  if (!gbridge) {
    vm.$router.push({path: '/'});
    return;
  }
  vm.source.status_bridge = gbridge;
}

export default {
  components: {
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
