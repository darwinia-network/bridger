<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Execution header</h2>
    </v-col>
    <v-col cols="12">
      <v-simple-table dense>
        <template v-slot:default>
          <thead>
          <tr>
            <th style="width: 50%">Title</th>
            <th>Value</th>
          </tr>
          </thead>
          <tbody>
          <tr>
            <td class="subtitle-2">Relayed slot</td>
            <td>
              <v-progress-linear v-if="loading.relayedHeader" :color="executionChain.color" indeterminate/>
              <span v-else>{{ source.relayedHeader.slot }}</span>
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Relayed period</td>
            <td>
              <v-progress-linear v-if="loading.relayedHeader" :color="executionChain.color" indeterminate/>
              <span v-else>{{ source.relayedPeriod }}</span>
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Relayed state root</td>
            <td>
              <v-progress-linear v-if="loading.relayedStateRoot" :color="executionChain.color" indeterminate/>
              <ellipsis-text v-else :text="source.relayedStateRoot"/>
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Latest block by slot</td>
            <td>
              <v-progress-linear v-if="loading.lastFinalizedBlock" :color="executionChain.color" indeterminate/>
              <external-explorer
                v-else
                :identity="source.lastFinalizedBlock.message.body.execution_payload.block_number"
                type="block"
                :chain="executionChain"
              />
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Latest state root by slot</td>
            <td>
              <v-progress-linear v-if="loading.lastFinalizedBlock" :color="executionChain.color" indeterminate/>
              <ellipsis-text v-else :text="source.lastFinalizedBlock.message.body.execution_payload.state_root"/>
            </td>
          </tr>
          </tbody>
        </template>
      </v-simple-table>
    </v-col>
  </v-row>
</template>

<script>

import ExternalExplorer from '@/components/widgets/external-explorer';
import EllipsisText from "@/components/widgets/ellipsis-text";

async function initState(vm) {
  vm.subscriber.relayInfo = setInterval(() => {
    queryRelayInfo(vm);
  }, 1000 * 15);
}

async function queryRelayInfo(vm) {
  const bridgeTarget = vm.evmChain.bridge_target[vm.executionChain.bridge_chain_name];
  const {lc_consensus, lc_execution} = bridgeTarget.contract;

  // query relayed header
  vm.loading.relayedHeader = true;
  vm.source.relayedHeader = await vm.evmClient
    .consensusLightClient(lc_consensus)
    .finalizedHeader();
  vm.source.relayedPeriod = vm.$eth2.toolkit.calcPeriod(vm.source.relayedHeader.slot);
  vm.loading.relayedHeader = false;

  // query consensus block
  vm.loading.lastFinalizedBlock = true;
  const lastFinalizedBlock = await vm.consensusClient.block(vm.source.relayedHeader.slot);
  vm.source.lastFinalizedBlock = lastFinalizedBlock.data;
  vm.loading.lastFinalizedBlock = false;

  vm.loading.relayedStateRoot = true;
  vm.source.relayedStateRoot = await vm.evmClient.executionLightClient(lc_execution)
    .stateRoot();
  vm.loading.relayedStateRoot = false;
}

export default {
  components: {EllipsisText, ExternalExplorer},
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
      relayedHeader: null,
      lastFinalizedBlock: null,
      relayedStateRoot: null,
    },
    loading: {
      relayedHeader: true,
      lastFinalizedBlock: true,
      relayedStateRoot: true,
    },
    subscriber: {
      relayInfo: null,
    },
  }),
  async created() {
    await queryRelayInfo(this);
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
