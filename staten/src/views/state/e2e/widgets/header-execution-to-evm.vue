<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Execution header</h2>
    </v-col>
    <v-col cols="12">
      <v-table density="compact">
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
      </v-table>
    </v-col>
  </v-row>
</template>


<script lang="ts" setup>

import {defineProps, inject, onBeforeUnmount, onMounted, PropType, reactive, toRefs} from 'vue'
import BigNumber from "bignumber.js";
import {Eth2Client} from "@/plugins/eth2";
import {BridgeEthereumChainInfo} from "@/types/app";
import {EvmClient} from "@/plugins/eth2/evm";
import {ExecutionClient} from "@/plugins/eth2/execution";
import {ConsensusClient} from "@/plugins/eth2/consensus";
import EllipsisText from "@/components/widgets/ellipsis-text.vue";
import ExternalExplorer from "@/components/widgets/external-explorer.vue";

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
  relayedHeader: string;
  lastFinalizedBlock: string;
  relayedStateRoot: string;
}

interface _StateLoading {
  relayedHeader: boolean;
  lastFinalizedBlock: boolean;
  relayedStateRoot: boolean;
}

const state = reactive({
  source: {} as _StateSource,
  loading: {
    relayedHeader: true,
    lastFinalizedBlock: true,
    relayedStateRoot: true,
  } as _StateLoading,
  subscriber: {
    relayInfo: null,
  },
});

const {source, loading, subscriber} = toRefs(state);

async function initState() {
  subscriber.value.relayInfo = setInterval(queryRelayInfo, 1000 * 15);
}


async function queryRelayInfo() {
  const {evmChain, consensusClient, evmClient, executionChain} = props;
  const bridgeTarget = evmChain.bridge_target[executionChain.bridge_chain_name];
  const {lc_consensus, lc_execution} = bridgeTarget.contract;

  // query relayed header
  loading.value.relayedHeader = true;
  source.value.relayedHeader = await evmClient
    .consensusLightClient(lc_consensus)
    .finalizedHeader();
  source.value.relayedPeriod = eth2.toolkit.calcPeriod(source.value.relayedHeader.slot);
  loading.value.relayedHeader = false;

  // query consensus block
  loading.value.lastFinalizedBlock = true;
  const lastFinalizedBlock = await consensusClient.block(source.relayedHeader.slot);
  source.value.lastFinalizedBlock = lastFinalizedBlock.data;
  loading.value.lastFinalizedBlock = false;

  loading.value.relayedStateRoot = true;
  source.value.relayedStateRoot = await evmClient.executionLightClient(lc_execution)
    .stateRoot();
  loading.value.relayedStateRoot = false;
}


onMounted(() => {
  queryRelayInfo();
  initState();
});


onBeforeUnmount(() => {
  // @ts-ignore
  subscriber.value.queryMessage && clearInterval(subscriber.value.queryMessage);
  subscriber.value.relayInfo && clearInterval(subscriber.value.relayInfo);
});

</script>

