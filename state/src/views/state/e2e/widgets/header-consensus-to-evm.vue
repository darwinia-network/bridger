<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Consensus header</h2>
    </v-col>
    <v-col cols="12">
      <v-simple-table dense>
        <template v-slot:default>
          <thead>
          <tr>
            <th style="width: 40%">Title</th>
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
            <td class="subtitle-2">Relayed slot</td>
            <td>
              <v-progress-linear v-if="loading.relayedHeader" :color="executionChain.color" indeterminate/>
              <span v-else>{{ source.relayedHeader.slot }}</span>
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Current period</td>
            <td>
              <v-progress-linear v-if="loading.lastHeader" :color="executionChain.color" indeterminate/>
              <span v-else>{{ source.relayedPeriod }}</span>
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Current slot</td>
            <td>
              <v-progress-linear v-if="loading.lastHeader" :color="executionChain.color" indeterminate/>
              <span v-else>{{ source.lastHeader.header.message.slot }}</span>
            </td>
          </tr>
          </tbody>
        </template>
      </v-simple-table>
    </v-col>
  </v-row>
</template>

<script>
import BigNumber from 'bignumber.js'

async function initState(vm) {
  vm.subscriber.relayInfo = setInterval(() => queryRelyInfo(vm), 1000 * 20);
}


async function queryRelyInfo(vm) {
  const bridgeTarget = vm.evmChain.bridge_target[vm.executionChain.bridge_chain_name];
  const consensusLightClientAddress = bridgeTarget.contract.lc_consensus;
  // query relayed header
  vm.loading.relayedHeader = true;
  vm.source.relayedHeader = await vm.evmClient
    .consensusLightClient(consensusLightClientAddress)
    .finalizedHeader();
  vm.source.relayedPeriod = vm.$eth2.toolkit.calcPeriod(vm.source.relayedHeader.slot);
  vm.loading.relayedHeader = false;

  // query last header from consensus chain
  vm.loading.lastHeader = true;
  const lastHeader = await vm.consensusClient.header('head');
  vm.source.lastHeader = lastHeader.data;
  vm.source.currentPeriod = vm.$eth2.toolkit.calcPeriod(new BigNumber(vm.source.lastHeader.header.message.slot));
  vm.loading.lastHeader = false;
}

export default {
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
      relayedSlot: null,
      currentPeriod: null,
      relayedHeader: null,
      lastHeader: null,
    },
    loading: {
      relayedHeader: true,
      lastHeader: true,
    },
    subscriber: {
      relayInfo: null,
    }
  }),
  async created() {
    await queryRelyInfo(this);
    await initState(this);
  },
  destroyed() {
    const vm = this;
    vm.subscriber.relayInfo && clearInterval(vm.subscriber.relayInfo);
  }
}
</script>

<style scoped>

</style>
