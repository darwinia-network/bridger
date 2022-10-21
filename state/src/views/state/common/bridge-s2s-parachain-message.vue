<template>
  <v-row>
    <v-col cols="12">
      <s2s-message-relay
        :key="`s2s-message-${paraChain.name}-${soloChain.name}`"
        :parachain-bridge="false"
        :source-client="paraClient"
        :target-client="soloClient"
        :source-chain="paraChain"
        :target-chain="soloChain"
      />
    </v-col>
    <v-col cols="12">
      <s2s-feemarket
        :key="`s2s-feemarket-${paraChain.name}-${soloChain.name}`"
        :source-client="paraClient"
        :source-chain="paraChain"
        :target-chain="soloChain"
      />
    </v-col>
  </v-row>
</template>

<script>
import S2sFeemarket from '@/views/state/common/widgets/s2s-feemarket'
import S2sMessageRelay from '@/views/state/common/widgets/s2s-message-relay';

export default {
  components: {S2sFeemarket, S2sMessageRelay},
  props: {
    soloClient: {
      type: Object,
    },
    paraClient: {
      type: Object,
    },
    relayClient: {
      type: Object,
    },
    soloChain: {
      type: Object,
    },
    paraChain: {
      type: Object,
    },
    relayChain: {
      type: Object,
    },
  },
  data: () => ({}),
  created() {
  }
}
</script>

<style scoped>

</style>
