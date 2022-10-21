<template>
  <v-card max-width="400">
    <v-card-title>
      <v-icon large left>mdi-twitter</v-icon>
      {{ chain.name }}x
    </v-card-title>
    <v-list-item>
      <v-list-item-content>
        <v-list-item-title>Finalized: #1349582</v-list-item-title>
      </v-list-item-content>
    </v-list-item>
  </v-card>
</template>

<script>
export default {
  props: {
    client: {
      type: Object,
    },
    chain: {
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
