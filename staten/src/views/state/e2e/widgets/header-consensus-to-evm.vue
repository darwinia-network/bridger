<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Consensus header</h2>
    </v-col>
    <v-col cols="12">
      <v-table density="compact">
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
      </v-table>
    </v-col>
  </v-row>
</template>


<script lang="ts" setup>

import {defineProps, inject, onBeforeUnmount, onMounted, PropType, reactive, toRefs} from 'vue'
import {BridgeEthereumChainInfo} from "@/types/app";
import {EvmClient} from "@/plugins/eth2/evm";
import {ExecutionClient} from "@/plugins/eth2/execution";
import {ConsensusClient} from "@/plugins/eth2/consensus";
import BigNumber from "bignumber.js";
import {Eth2Client} from "@/plugins/eth2";


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
  relayedPeriod: string;
  relayedSlot: string;
  currentPeriod: string;
  relayedHeader: string;
  lastHeader: string;
}

interface _StateLoading {
  relayedHeader: boolean,
  lastHeader: boolean,
}

const state = reactive({
  source: {} as _StateSource,
  loading: {
    relayedHeader: true,
    lastHeader: true,
  } as _StateLoading,
  subscriber: {
    relayInfo: null,
  }
});

const {source, loading, subscriber} = toRefs(state);

async function initState() {
  subscriber.value.relayInfo = setInterval(queryRelyInfo, 1000 * 20);
}


async function queryRelyInfo() {
  const {evmChain, executionChain, consensusClient, evmClient} = props;
  const bridgeTarget = evmChain.bridge_target[executionChain.bridge_chain_name];
  const consensusLightClientAddress = bridgeTarget.contract.lc_consensus;
  // query relayed header
  loading.value.relayedHeader = true;
  source.value.relayedHeader = await evmClient
    .consensusLightClient(consensusLightClientAddress)
    .finalizedHeader();
  source.value.relayedPeriod = eth2.toolkit.calcPeriod(source.value.relayedHeader.slot);
  loading.value.relayedHeader = false;

  // query last header from consensus chain
  loading.value.lastHeader = true;
  const lastHeader = await consensusClient.header('head');
  source.value.lastHeader = lastHeader.data;
  source.value.currentPeriod = eth2.toolkit.calcPeriod(new BigNumber(source.value.lastHeader.header.message.slot));
  loading.value.lastHeader = false;
}


onMounted(() => {
  initState();
  queryRelyInfo();
});


onBeforeUnmount(() => {
  // @ts-ignore
  subscriber.value.relayInfo && clearInterval(subscriber.value.relayInfo);
});
</script>

