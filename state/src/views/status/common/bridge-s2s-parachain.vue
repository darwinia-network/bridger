<template>
  <v-row>
    <v-col md="6" sm="12" xs="12">
      <s2s-header-relay
        :key="`s2s-header-${relayChain.name}-${soloChain.name}`"
        :parachain-bridge="true"
        :grandpa-pallet-name="soloChain.bridge_target[paraChain.bridge_chain_name].query_name.grandpa"
        :source-client="relayClient"
        :target-client="soloClient"
        :source-chain="relayChain"
        :target-chain="soloChain"
      />
    </v-col>
    <v-col md="6" sm="12" xs="12">
      <s2s-feemarket
        :key="`s2s-feemarket-${paraChain.name}-${soloChain.name}`"
        :parachain-bridge="true"
        :source-client="paraClient"
        :source-chain="paraChain"
        :target-chain="soloChain"
      />
    </v-col>
  </v-row>
</template>

<script>
import S2sHeaderRelay from '@/views/status/common/widgets/s2s-header-relay'
import S2sFeemarket from '@/views/status/common/widgets/s2s-feemarket'

export default {
  components: {S2sFeemarket, S2sHeaderRelay},
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
