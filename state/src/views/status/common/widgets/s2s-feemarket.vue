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
              <external-subscan :identity="relayer.id" type="account" :chain="sourceChain" />
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

import { BN } from '@polkadot/util'
import ExternalSubscan from '@/components/widgets/external-subscan';

async function initState(vm) {
  const bridgeTarget = vm.sourceChain.bridge_target[vm.targetChain.bridge_chain_name];
  vm.subscriber.assignedRelayers = await vm.sourceClient.query[bridgeTarget.query_name.feemarket]
    .assignedRelayers(v => {
      vm.source.ampleRelayers = v.isSome;
      vm.source.assignedRelayers = v.toJSON();
      vm.loading.assignedRelayers = false;
      if (vm.parachainBridge) {
        vm.source.assignedRelayers = vm.$stream(vm.source.assignedRelayers)
        .map(item => {
          return {
            ...item,
            collateral: new BN(vm.source.assignedRelayers[0].collateral.replace('0x', ''), 16).toString(),
            fee: new BN(vm.source.assignedRelayers[0].fee.replace('0x', ''), 16).toString(),
          }
        })
        .toArray();
      }
    });
}

export default {
  components: {ExternalSubscan},
  props: {
    parachainBridge: {
      type: Boolean,
      default: false,
    },
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
