<template>
  <v-row>
    <v-col cols="12">
      <sync-committee
        :evm-chain="evmChain"
        :ethereum-chain="ethereumChain"
        :evm-client="evmClient"
        :ethereum-client="ethereumClient"
      />
    </v-col>
  </v-row>
</template>

<script>
import SyncCommittee from '@/views/state/e2e/widgets/sync-committee';

export default {
  components: {SyncCommittee},
  props: {
    evmChain: {
      type: Object,
    },
    ethereumChain: {
      type: Object,
    },
    evmClient: {
      type: Object,
    },
    ethereumClient: {
      type: Object,
    },
  },
  data: () => ({

  }),
  created() {
  }
}
</script>

<style scoped>

</style>
