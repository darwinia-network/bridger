<template>
  <v-row>
    <v-col cols="12">
      <v-tabs v-model="source.tab">
        <v-tabs-slider :color="sourceChain.color"/>
        <v-tab key="tab-relay-header">Header</v-tab>
        <v-tab key="tab-relay-message">Message</v-tab>
        <v-tab key="tab-feemarket">Feemarket</v-tab>
      </v-tabs>
    </v-col>
    <v-col cols="12" v-if="source.tab === 0">
      <s2s-header-relay
        :key="`s2s-header-${sourceChain.name}-${targetChain.name}`"
        :source-client="sourceClient"
        :target-client="targetClient"
        :source-chain="sourceChain"
        :target-chain="targetChain"
      />
    </v-col>
    <v-col cols="12" v-if="source.tab === 1">
      <s2s-message-relay
        :key="`s2s-message-${sourceChain.name}-${targetChain.name}`"
        :source-client="sourceClient"
        :target-client="targetClient"
        :source-chain="sourceChain"
        :target-chain="targetChain"
      />
    </v-col>
    <v-col cols="12" v-if="source.tab === 2">
      <s2s-feemarket
        :key="`s2s-feemarket-${sourceChain.name}-${targetChain.name}`"
        :source-client="sourceClient"
        :source-chain="sourceChain"
        :target-chain="targetChain"
      />
    </v-col>

<!--    <v-col cols="12">
      <v-divider/>
    </v-col>-->
  </v-row>
</template>

<script>

import S2sFeemarket from '@/views/status/common/widgets/s2s-feemarket';
import S2sMessageRelay from '@/views/status/common/widgets/s2s-message-relay';
import S2sHeaderRelay from '@/views/status/common/widgets/s2s-header-relay';
import S2sPanelChain from '@/views/status/common/panel/s2s-panel-chain';

async function initState(vm) {
}

export default {
  props: {
    sourceClient: {
      type: Object,
    },
    targetClient: {
      type: Object,
    },
    sourceChain: {
      type: Object,
    },
    targetChain: {
      type: Object,
    },
  },
  components: {S2sPanelChain, S2sHeaderRelay, S2sMessageRelay, S2sFeemarket},
  data: () => ({
    source: {
      substrate: {
        sourceClient: null,
        targetClient: null,
      },
      tab: 1,
    },
  }),
  created() {
    initState(this);
  },
}
</script>

<style scoped>

</style>
