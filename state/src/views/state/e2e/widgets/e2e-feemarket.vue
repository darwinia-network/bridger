<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Feemarket</h2>
    </v-col>
    <v-col cols="12" v-if="source.ampleRelayers">
      <v-simple-table dense>
        <template v-slot:default>
          <thead>
          <tr>
            <th class="text-left" style="width: 50%;">Account</th>
            <th class="text-left">Balance</th>
            <th class="text-left">Locked</th>
            <th class="text-left">Fee ({{ sourceChain.currency }})</th>
          </tr>
          </thead>
          <tbody>
          <tr v-if="loading.assignedRelayers">
            <td>
              <v-progress-linear :color="sourceChain.color" indeterminate/>
            </td>
            <td>
              <v-progress-linear :color="sourceChain.color" indeterminate/>
            </td>
            <td>
              <v-progress-linear :color="sourceChain.color" indeterminate/>
            </td>
            <td>
              <v-progress-linear :color="sourceChain.color" indeterminate/>
            </td>
          </tr>
          <tr v-for="(relayer, ix) in source.assignedRelayers" :key="relayer.id">
            <td>
              <external-explorer :identity="relayer.id" type="address" :chain="sourceChain"/>
            </td>
            <td><code>{{ relayer.balance }}</code></td>
            <td><code>{{ relayer.locked }}</code></td>
            <td>
              <code :class="{'green--text': ix === source.assignedRelayers.length - 1}" v-text="relayer.fee"/>
            </td>
          </tr>
          </tbody>
        </template>
      </v-simple-table>
    </v-col>
    <v-col cols="12" v-else>
      <p>No assigned relayers</p>
    </v-col>
  </v-row>
</template>

<script>

import ExternalExplorer from "@/components/widgets/external-explorer";
import BigNumber from "bignumber.js";

async function initState(vm) {
  vm.loading.assignedRelayers = true;
  const bridgeTarget = vm.sourceChain.bridge_target[vm.targetChain.bridge_chain_name];
  const assignedRelayers = await vm.sourceClient
    .feemarket(bridgeTarget.contract.feemarket)
    .assignedRelayers();
  vm.loading.assignedRelayers = false;
  if (!assignedRelayers || assignedRelayers.isEmpty) {
    vm.source.ampleRelayers = false;
    return;
  }
  vm.source.ampleRelayers = true;
  const precision = new BigNumber(10).pow(vm.sourceChain.precisionEvm || vm.sourceChain.precision);
  vm.source.assignedRelayers = vm.$stream(assignedRelayers)
    .map(item => {
      const balance = new BigNumber(item.balance.toString()).div(precision);
      const locked = new BigNumber(item.locked.toString()).div(precision);
      const fee = new BigNumber(item.fee.toString()).div(precision);
      return {...item, balance, locked, fee};
    }).toArray();
}

export default {
  components: {ExternalExplorer},
  props: {
    sourceChain: {
      type: Object,
    },
    sourceClient: {
      type: Object,
    },
    targetChain: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      ampleRelayers: true,
      assignedRelayers: null,
    },
    loading: {
      assignedRelayers: true,
    },
  }),
  created() {
    initState(this);
  }
}
</script>

<style scoped>

</style>
