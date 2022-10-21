<template>
  <v-row>
    <v-col md="6" sm="12" xs="12">
      <s2s-header-relay
        :key="`s2s-header-${sourceChain.name}-${targetChain.name}`"
        :source-client="sourceClient"
        :target-client="targetClient"
        :source-chain="sourceChain"
        :target-chain="targetChain"
      />
    </v-col>
    <v-col md="6" sm="12" xs="12">
      <s2s-feemarket
        :key="`s2s-feemarket-${sourceChain.name}-${targetChain.name}`"
        :source-client="sourceClient"
        :source-chain="sourceChain"
        :target-chain="targetChain"
      />
    </v-col>
    <v-col cols="12">
      <s2s-message-relay
        :key="`s2s-message-${sourceChain.name}-${targetChain.name}`"
        :parachain-bridge="parachainBridge"
        :source-client="sourceClient"
        :target-client="targetClient"
        :source-chain="sourceChain"
        :target-chain="targetChain"
      />
    </v-col>


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
    parachainBridge: {
      type: Boolean,
      default: false,
    },
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
    },
  }),
  created() {
    initState(this);
  },
}
</script>

<style scoped>

</style>
