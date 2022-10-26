<template>
  <v-row>
    <v-col cols="12">
      <v-simple-table dense>
        <template v-slot:default>
          <thead>
          <tr>
            <th style="width: 30%">Title</th>
            <th>Value</th>
          </tr>
          </thead>
          <tbody>
          <tr>
            <td class="subtitle-2">Relayed period</td>
            <td>
              <v-progress-linear v-if="loading.relayedHeader" :color="executionChain.color" indeterminate/>
              <span v-else>{{ source.relayedPeriod }}</span>
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Relayed committee roots</td>
            <td>
              <v-progress-linear v-if="loading.currentSyncCommitteeRoots" :color="executionChain.color" indeterminate/>
              <ellipsis-text v-else :text="source.currentSyncCommitteeRoots"/>
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Next committee roots</td>
            <td>
              <v-progress-linear v-if="loading.nextSyncCommitteeRoots" :color="executionChain.color" indeterminate/>
              <ellipsis-text v-else :text="source.nextSyncCommitteeRoots"/>
            </td>
          </tr>
          </tbody>
        </template>
      </v-simple-table>
    </v-col>
  </v-row>
</template>

<script>

import EllipsisText from '@/components/widgets/ellipsis-text';

async function initState(vm) {
  const bridgeTarget = vm.evmChain.bridge_target[vm.executionChain.bridge_chain_name];
  const consensusLightClientAddress = bridgeTarget.contract.lc_consensus;
  const header = await vm.evmClient.consensusLightClient(consensusLightClientAddress)
    .finalizedHeader();
  vm.loading.relayedHeader = false;
  const period = vm.$eth2.toolkit.calcPeriod(header.slot);
  vm.source.relayedPeriod = period;
  vm.source.currentSyncCommitteeRoots = await vm.evmClient
    .consensusLightClient(consensusLightClientAddress)
    .syncCommitteeRoots(period);
  vm.loading.currentSyncCommitteeRoots = false;
  vm.source.nextSyncCommitteeRoots = await vm.evmClient
    .consensusLightClient(consensusLightClientAddress)
    .syncCommitteeRoots(period.add(1));
  vm.loading.nextSyncCommitteeRoots = false;
}

export default {
  components: {EllipsisText},
  props: {
    evmChain: {
      type: Object,
    },
    executionChain: {
      type: Object,
    },
    consensusChain: {
      type: Object,
    },
    evmClient: {
      type: Object,
    },
    executionClient: {
      type: Object,
    },
    consensusClient: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      relayedPeriod: null,
      currentSyncCommitteeRoots: null,
      nextSyncCommitteeRoots: null,
    },
    loading: {
      relayedHeader: true,
      currentSyncCommitteeRoots: true,
      nextSyncCommitteeRoots: true,
    },
  }),
  created() {
    initState(this);
  }
}
</script>

<style scoped>

</style>
