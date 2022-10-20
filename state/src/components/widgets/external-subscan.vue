<template>
  <v-row>
    <v-tooltip bottom>
      <template v-slot:activator="{ on, attrs }">
        <v-col cols="8" v-bind="attrs" v-on="on">
          <vue-ellipsis :text="identity"/>
        </v-col>
      </template>
      <span v-text="identity"/>
    </v-tooltip>
    <v-col cols="4">
      <v-tooltip bottom>
        <template v-slot:activator="{ on, attrs }">
          <v-btn x-small icon v-bind="attrs" v-on="on" v-clipboard:copy="identity" v-clipboard:success="onCopy">
            <v-icon x-small>mdi-content-copy</v-icon>
          </v-btn>
        </template>
        <span>Copy</span>
      </v-tooltip>
      <v-tooltip bottom v-if="chain.explorer">
        <template v-slot:activator="{ on, attrs }">
          <v-btn x-small icon :href="`${chain.explorer}/${type}/${identity}`" target="_blank" v-bind="attrs" v-on="on">
            <v-icon x-small>mdi-open-in-new</v-icon>
          </v-btn>
        </template>
        <span>Explorer</span>
      </v-tooltip>
      <v-btn v-else x-small icon disabled>
        <v-icon x-small>mdi-open-in-new</v-icon>
      </v-btn>
    </v-col>
  </v-row>
</template>

<script>
export default {
  props: {
    identity: {
      type: String | Number,
    },
    type: {
      type: String,
      default: 'block',
    },
    chain: {
      type: Object,
    },
  },
  data: () => ({}),
  methods: {
    onCopy() {
      this.$toast.info('Copied');
    }
  },
  created() {
  }
}
</script>

<style scoped>
.box-identity {

}
</style>
