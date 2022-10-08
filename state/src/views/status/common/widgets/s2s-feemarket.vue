<template>
  <v-row>
    <v-col cols="12" v-if="source.feemarket.ampleRelayers">
      <v-simple-table dense>
        <template v-slot:default>
          <thead>
          <tr>
            <th class="text-left">Account</th>
            <th class="text-left">Collateral</th>
            <th class="text-left">Fee ({{ sourceChain.currency }})</th>
          </tr>
          </thead>
          <tbody>
          <tr v-if="loading.feemarket">
            <td><v-progress-linear :color="sourceChain.color" indeterminate/></td>
            <td><v-progress-linear :color="sourceChain.color" indeterminate/></td>
            <td><v-progress-linear :color="sourceChain.color" indeterminate/></td>
          </tr>
          <tr v-for="(relayer, ix) in source.feemarket.assignedRelayers" :key="relayer.id">
            <td>
              <code>{{ relayer.id }}</code>
              <v-btn icon small :href="`${sourceChain.explorer}/account/${relayer.id}`" target="_blank">
                <v-icon small>mdi-open-in-new</v-icon>
              </v-btn>
            </td>
            <td><code>{{ relayer.collateral }}</code></td>
            <td>
              <code :class="{'green--text': ix === source.feemarket.assignedRelayers.length - 1}" v-text="relayer.fee"/>
            </td>
          </tr>
          </tbody>
        </template>
      </v-simple-table>
    </v-col>
    <v-col cols="12" v-else>
      <p>No assigned relayers</p>
    </v-col>
    <v-col cols="12">
      <v-alert text color="grey accent-4" icon="mdi-information">
        More feemarket operation please visit
        <v-btn text small href="https://feemarket.darwinia.network" target="_blank">
          feemarket ui
        </v-btn>
        project.
      </v-alert>
      <p>
      </p>
    </v-col>
  </v-row>
</template>

<script>

async function initState(vm) {
  vm.loading.feemarket = true;
  const bridgeTarget = vm.sourceChain.bridge_target[vm.targetChain.bridge_chain_name];
  const relayers = await vm.sourceClient.query[bridgeTarget.query_name.feemarket].assignedRelayers();
  vm.source.feemarket.ampleRelayers = relayers.isSome;
  vm.source.feemarket.assignedRelayers = relayers.toHuman();
  vm.loading.feemarket = false;
}

export default {
  props: {
    sourceClient: {
      type: Object,
    },
    sourceChain: {
      type: Object,
    },
    targetChain: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      feemarket: {
        ampleRelayers: true,
        assignedRelayers: [],
      },
    },
    loading: {
      feemarket: false,
    }
  }),
  created() {
    initState(this);
  }
}
</script>

<style scoped>

</style>
