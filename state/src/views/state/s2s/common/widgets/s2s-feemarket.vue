<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Fee market</h2>
    </v-col>
    <v-col cols="12" v-if="source.ampleRelayers">
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
          </tr>
          <tr v-for="(relayer, ix) in source.assignedRelayers" :key="relayer.id">
            <td>
              <external-explorer :identity="relayer.id" type="account" :chain="sourceChain"/>
            </td>
            <td><code>{{ relayer.collateral }}</code></td>
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

import BigNumber from 'bignumber.js'
import ExternalExplorer from '@/components/widgets/external-explorer';

async function initState(vm) {
  const bridgeTarget = vm.sourceChain.bridge_target[vm.targetChain.bridge_chain_name];
  vm.subscriber.assignedRelayers = await vm.sourceClient.query[bridgeTarget.query_name.feemarket]
    .assignedRelayers(v => {
      vm.source.ampleRelayers = v.isSome;
      vm.source.assignedRelayers = v.toJSON();
      vm.loading.assignedRelayers = false;
      const precision = new BigNumber(10).pow(vm.sourceChain.precision);
      vm.source.assignedRelayers = vm.$stream(vm.source.assignedRelayers)
        .map(item => {
          const collateral = new BigNumber(item.collateral);
          const fee = new BigNumber(item.fee);
          return {
            ...item,
            collateral: collateral.div(precision),
            fee: fee.div(precision),
          }
        })
        .toArray();
    });
}

export default {
  components: {ExternalExplorer},
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
      ampleRelayers: true,
      assignedRelayers: [],
    },
    subscriber: {
      assignedRelayers: null,
    },
    loading: {
      assignedRelayers: true,
    }
  }),
  created() {
    initState(this);
  },
  destroyed() {
    const vm = this;
    vm.subscriber.assignedRelayers && vm.subscriber.assignedRelayers();
  }
}
</script>

<style scoped>

</style>
