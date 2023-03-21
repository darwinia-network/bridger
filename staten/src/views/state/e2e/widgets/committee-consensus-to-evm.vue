<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Sync committee</h2>
    </v-col>
    <v-col cols="12">
      <v-table density="compact">
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
      </v-table>
    </v-col>
  </v-row>
</template>



<script lang="ts" setup>

import {defineProps, inject, onMounted, PropType, reactive, toRaw, toRefs} from 'vue'
import {EvmClient} from "@/plugins/eth2/evm";
import {ExecutionClient} from "@/plugins/eth2/execution";
import {ConsensusClient} from "@/plugins/eth2/consensus";
import {BridgeEthereumChainInfo} from "@/types/app";
import {Eth2Client} from "@/plugins/eth2";
import BigNumber from "bignumber.js";
import EllipsisText from "@/components/widgets/ellipsis-text.vue";

const eth2 = inject('eth2') as Eth2Client;

const props = defineProps({
  evmChain: {
    type: Object as PropType<BridgeEthereumChainInfo>,
  },
  executionChain: {
    type: Object as PropType<BridgeEthereumChainInfo>,
  },
  consensusChain: {
    type: Object as PropType<BridgeEthereumChainInfo>,
  },
  evmClient: {
    type: Object as PropType<EvmClient>,
  },
  executionClient: {
    type: Object as PropType<ExecutionClient>,
  },
  consensusClient: {
    type: Object as PropType<ConsensusClient>,
  },
});

interface _StateSource {
  relayedPeriod: BigNumber;
  currentSyncCommitteeRoots: string;
  nextSyncCommitteeRoots: string;
}

interface _StateLoading {
  relayedHeader: boolean;
  currentSyncCommitteeRoots: boolean;
  nextSyncCommitteeRoots: boolean;
}

const state = reactive({
  source: {} as _StateSource,
  loading: {
    relayedHeader: false,
    currentSyncCommitteeRoots: false,
    nextSyncCommitteeRoots: false,
  } as _StateLoading,
});

const {source, loading} = toRefs(state);

async function initState() {
  const {evmChain, executionChain} = props;
  const bridgeTarget = evmChain?.bridge_target[executionChain.bridge_chain_name];
  const consensusLightClientAddress = bridgeTarget.contract.lc_consensus;

  const evmClient = toRaw(props.evmClient);

  const header = await evmClient?.consensusLightClient(consensusLightClientAddress)
    .finalizedHeader();
  loading.value.relayedHeader = false;
  const period = eth2.toolkit.calcPeriod(header.slot);
  source.value.relayedPeriod = period;
  source.value.currentSyncCommitteeRoots = await evmClient
    .consensusLightClient(consensusLightClientAddress)
    .syncCommitteeRoots(period);
  loading.value.currentSyncCommitteeRoots = false;
  source.value.nextSyncCommitteeRoots = await evmClient
    .consensusLightClient(consensusLightClientAddress)
    .syncCommitteeRoots(period.add(1));
  loading.value.nextSyncCommitteeRoots = false;
}


onMounted(() => {
  initState();
});


</script>
