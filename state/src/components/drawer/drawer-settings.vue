<template>
  <v-container>
    <v-row>
      <v-col cols="12" md="12" sm="6">
        <v-checkbox v-model="dest.form.enableTestnet" label="Allow testnet" @change="changedData"/>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
export default {
  data: () => ({
    dest: {
      form: {
        enableTestnet: true,
      }
    }
  }),
  methods: {
    changedData() {
      const vm = this;
      vm.$emit('change', vm.dest.form);
    }
  },
  created() {
    const vm = this;
    vm.$emit('change', vm.dest.form);
  }
}
</script>

<style scoped>

</style>
