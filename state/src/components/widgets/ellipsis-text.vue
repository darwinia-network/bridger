<template>
  <v-row>
    <v-tooltip bottom>
      <template v-slot:activator="{ on, attrs }">
        <v-col cols="9" v-bind="attrs" v-on="on">
          <vue-ellipsis :text="text"/>
        </v-col>
      </template>
      <span v-text="text"/>
    </v-tooltip>
    <v-col cols="3">
      <v-tooltip bottom>
        <template v-slot:activator="{ on, attrs }">
          <v-btn x-small icon v-bind="attrs" v-on="on" v-clipboard:copy="text" v-clipboard:success="onCopy">
            <v-icon x-small>mdi-content-copy</v-icon>
          </v-btn>
        </template>
        <span>Copy</span>
      </v-tooltip>
    </v-col>
  </v-row>
</template>

<script>
export default {
  props: {
    text: {
      type: String | Number,
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

