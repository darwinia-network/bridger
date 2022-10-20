<template>
  <v-row>
    <v-col md="6" sm="12" xs="12">
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
import S2sFeemarket from '@/views/status/common/widgets/s2s-feemarket'

export default {
  components: {S2sFeemarket},
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
