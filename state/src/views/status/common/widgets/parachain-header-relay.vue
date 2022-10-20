<template>
  <v-row>
    <v-col cols="12">
      parachain header relay
    </v-col>
  </v-row>
</template>

<script>
export default {
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
    targetChain: {
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
