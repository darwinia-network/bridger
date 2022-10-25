<template>
  <v-row>
    <v-col cols="12">
      Sync committee
      <pre>{{source}}</pre>
    </v-col>
  </v-row>
</template>

<script>

async function initState(vm) {
  const beaconLightClientAddress = '0x9C266C48F07121181d8424768f0deD0170cC63A6';
  const header = await vm.evmClient.beaconLightClient(beaconLightClientAddress)
    .finalizedHeader();
  const period = header.slot.div(32).div(256);
  vm.source.currentSyncCommitteeRoots = await vm.evmClient.beaconLightClient(beaconLightClientAddress)
    .syncCommitteeRoots(period);
  vm.loading.currentSyncCommitteeRoots = false;
  vm.source.nextSyncCommitteeRoots = await vm.evmClient.beaconLightClient(beaconLightClientAddress)
    .syncCommitteeRoots(period.add(1));
  vm.loading.nextSyncCommitteeRoots = false;
}

export default {
  props: {
    evmChain: {
      type: Object,
    },
    ethereumChain: {
      type: Object,
    },
    evmClient: {
      type: Object,
    },
    ethereumClient: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      currentSyncCommitteeRoots: null,
      nextSyncCommitteeRoots: null,
    },
    loading: {
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
