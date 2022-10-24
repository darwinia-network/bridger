<template>
  <v-row>
    <v-col md="6" sm="12" xs="12">
      <parachain-header-relay
        :solo-client="soloClient"
        :para-client="paraClient"
        :relay-client="relayClient"
        :solo-chain="soloChain"
        :para-chain="paraChain"
        :relay-chain="relayChain"
      />
    </v-col>
    <v-col md="6" sm="12" xs="12">
      <parahead-relay
        :solo-client="soloClient"
        :para-client="paraClient"
        :relay-client="relayClient"
        :solo-chain="soloChain"
        :para-chain="paraChain"
        :relay-chain="relayChain"
      />
    </v-col>
  </v-row>
</template>

<script>

import ParachainHeaderRelay from "@/views/state/s2s/common/widgets/parachain-header-relay";
import ParaheadRelay from "@/views/state/s2s/common/widgets/parahead-relay";

export default {
  components: {ParaheadRelay, ParachainHeaderRelay},
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
