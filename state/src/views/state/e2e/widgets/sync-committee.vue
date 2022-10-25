<template>
  <v-row>
    <v-col cols="12">
      Sync committee
    </v-col>
  </v-row>
</template>

<script>

async function initState(vm) {
  const beaconLightClientAddress = '0x9C266C48F07121181d8424768f0deD0170cC63A6';
  const header = await vm.evmClient.beaconLightClient(beaconLightClientAddress)
    .finalizedHeader();
  const syncCommitteeRoots = await vm.evmClient.beaconLightClient(beaconLightClientAddress)
    .syncCommitteeRoots(header.slot.div(32).div(256));
  console.log(syncCommitteeRoots);
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
  data: () => ({}),
  created() {
    initState(this);
  }
}
</script>

<style scoped>

</style>
