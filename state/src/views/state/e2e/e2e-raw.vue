<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <pre>{{ bridge }}</pre>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
export default {
  props: {
    bridge: {
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
